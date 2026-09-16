set shell := ["bash", "-euo", "pipefail", "-c"]

# List recipes
default:
    @just --list

# ---------------------------------------------------------------- workflow --

# Uses docs/briefs/SLUG.md if the Architect drafted one, else the blank template.
# Create branch + worktree + task brief in one command (human only)
task slug role="dev":
    git fetch origin main
    git worktree add -b {{role}}/{{slug}} ../wt-{{slug}} origin/main
    mkdir -p ../wt-{{slug}}/.task
    if [ -f "docs/briefs/{{slug}}.md" ]; then src="docs/briefs/{{slug}}.md"; else src="docs/templates/BRIEF.md"; fi; sed -e "s/[{][{]SLUG[}][}]/{{slug}}/g" -e "s/[{][{]ROLE[}][}]/{{role}}/g" "$src" > ../wt-{{slug}}/.task/BRIEF.md; echo "   brief from: $src"
    @echo "→ ../wt-{{slug}} — review .task/BRIEF.md, then: cd ../wt-{{slug}} && claude"

# Tear down worktree + local branch after the PR is squash-merged (human only)
land slug:
    git worktree remove ../wt-{{slug}} --force
    for b in $(git branch --list "*/{{slug}}" | tr -d ' *+'); do git branch -D "$b"; done
    git fetch --prune origin
    git pull --ff-only origin main

# Show all active task worktrees
tasks:
    git worktree list

# ------------------------------------------------------------------ checks --

# fmt + clippy + test — run before every commit
verify: fmt-check lint test

fmt-check:
    cargo fmt --all --check

# Apply formatting
fmt:
    cargo fmt --all

lint:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
    cargo test --workspace

# NOTE: the SKIP branch exists only until the Phase 0 harness lands.
# The harness PR must delete it (tracked in docs/plan/phase-0-validation.md).

# Numerical regression vs tests/fixtures/published
regress:
    if [ ! -f crates/ballistics/tests/published_reference.rs ]; then echo "SKIPPED: regression harness not created yet (Phase 0 task). This skip must be removed by that PR."; exit 0; fi; cargo test -p ballistics --test published_reference -- --nocapture

# Licence + advisory check
deny:
    cargo deny check

# What the PR pipeline runs. Run this before opening a PR.
ci-local: verify deny regress

# ------------------------------------------------------------------- gates --
# Custom PR gates (docs/agents/ci.md). Each is reproducible locally against
# any base ref. pr.yml runs these after `just ci-local`.

# Run all three custom gates against base (default: origin/main)
gates base="origin/main": (gate-diff-size base) (gate-test-integrity base) (gate-provenance base)

# Fail if the diff exceeds 400 changed lines, excluding tests/, fixtures/, *.lock
gate-diff-size base="origin/main":
    #!/usr/bin/env bash
    set -euo pipefail
    limit=400
    changed=$(git diff --numstat {{base}}...HEAD -- . \
        ':(exclude,glob)**/tests/**' ':(exclude,glob)tests/**' \
        ':(exclude,glob)**/fixtures/**' ':(exclude,glob)fixtures/**' \
        ':(exclude,glob)**/*.lock' \
      | awk '{a+=$1; d+=$2} END {print a+d+0}')
    echo "gate-diff-size: ${changed} changed lines vs {{base}} (excl. tests/, fixtures/, *.lock; limit ${limit})"
    if [ "${changed}" -gt "${limit}" ]; then
      echo "FAIL: diff is ${changed} lines, over the ${limit}-line budget."
      echo "Why: CLAUDE.md 'Diff budget' — small diffs stay reviewable and revertible for a solo maintainer."
      echo "Fix: split the PR — commit what's coherent, open it, and write the remainder as a follow-up"
      echo "     brief in docs/briefs/. Only the human can bypass this, via the 'large-diff-approved' label."
      echo "Reproduce locally: just gate-diff-size"
      exit 1
    fi
    echo "OK"

# Fail if the diff removes a #[test], adds #[ignore], or changes a numeric
# literal under tests/
gate-test-integrity base="origin/main":
    #!/usr/bin/env bash
    set -euo pipefail
    fail=0
    echo "gate-test-integrity: checking vs {{base}}"

    removed=$(git diff {{base}}...HEAD -- '*.rs' | grep -c '^-.*#\[test\]' || true)
    added=$(git diff {{base}}...HEAD -- '*.rs' | grep -c '^+.*#\[test\]' || true)
    if [ "${removed:-0}" -gt "${added:-0}" ]; then
      echo "FAIL: diff removes ${removed} #[test] attribute(s) and adds only ${added}."
      echo "Why: CLAUDE.md — never modify, weaken, skip, or delete a test to make it pass."
      echo "Fix: restore the test. If it is genuinely wrong, stop and ask the human — do not delete it."
      echo "Reproduce locally: just gate-test-integrity"
      fail=1
    fi

    ignored=$(git diff {{base}}...HEAD -- '*.rs' | grep -c '^+.*#\[ignore' || true)
    if [ "${ignored:-0}" -gt 0 ]; then
      echo "FAIL: diff adds ${ignored} '#[ignore]' attribute(s)."
      echo "Why: CLAUDE.md — never modify, weaken, skip, or delete a test to make it pass."
      echo "Fix: fix the underlying test instead of ignoring it, or ask the human."
      echo "Reproduce locally: just gate-test-integrity"
      fail=1
    fi

    literal_changes=$(git diff --unified=0 {{base}}...HEAD -- '**/tests/**' 'tests/**' 2>/dev/null | awk '
      /^diff --git/ { have_rem=0; file=$0; sub(/^diff --git a\/.* b\//, "", file) }
      /^@@/          { have_rem=0; next }
      /^-[^-]/ {
        rem_raw=substr($0,2); rem_key=rem_raw; gsub(/[0-9]+(\.[0-9]+)?/, "#", rem_key)
        have_rem=1; next
      }
      /^\+[^+]/ {
        if (have_rem==1) {
          add_raw=substr($0,2); add_key=add_raw; gsub(/[0-9]+(\.[0-9]+)?/, "#", add_key)
          if (add_key==rem_key && add_raw!=rem_raw) print file": " rem_raw " -> " add_raw
        }
        have_rem=0
      }
    ')
    if [ -n "${literal_changes}" ]; then
      echo "FAIL: diff changes a numeric literal under tests/:"
      echo "${literal_changes}"
      echo "Why: CLAUDE.md — never change numerical tolerances in tests/. Tolerances are physics decisions, not an agent's call."
      echo "Fix: revert the literal, or ask the human."
      echo "Reproduce locally: just gate-test-integrity"
      fail=1
    fi

    if [ "${fail}" -ne 0 ]; then exit 1; fi
    echo "OK"

# Fail if crates/ballistics changed without a new docs/handoff/ file and a
# docs/CHANGELOG.md line
gate-provenance base="origin/main":
    #!/usr/bin/env bash
    set -euo pipefail
    ballistics_changed=$(git diff --name-only {{base}}...HEAD -- crates/ballistics | wc -l | tr -d ' ')
    echo "gate-provenance: crates/ballistics touched in ${ballistics_changed} file(s) vs {{base}}"
    if [ "${ballistics_changed}" -eq 0 ]; then
      echo "OK (crates/ballistics unchanged)"
      exit 0
    fi

    new_handoff=$(git diff --name-only --diff-filter=A {{base}}...HEAD -- docs/handoff | grep -vc '\.gitkeep$' || true)
    changelog_touched=$(git diff --name-only {{base}}...HEAD -- docs/CHANGELOG.md | wc -l | tr -d ' ')
    fail=0
    if [ "${new_handoff:-0}" -eq 0 ]; then
      echo "FAIL: crates/ballistics changed but no new docs/handoff/*.md file was added."
      echo "Why: CLAUDE.md provenance rule — every ballistics change needs a handoff note for the next session."
      echo "Fix: add docs/handoff/YYYY-MM-DD-<slug>.md (template: docs/templates/HANDOFF.md)."
      echo "Reproduce locally: just gate-provenance"
      fail=1
    fi
    if [ "${changelog_touched}" -eq 0 ]; then
      echo "FAIL: crates/ballistics changed but docs/CHANGELOG.md was not updated."
      echo "Why: CLAUDE.md provenance rule — every ballistics change needs a changelog line."
      echo "Fix: append one line to docs/CHANGELOG.md."
      echo "Reproduce locally: just gate-provenance"
      fail=1
    fi
    if [ "${fail}" -ne 0 ]; then exit 1; fi
    echo "OK"

bench:
    cargo bench -p ballistics

# Build Linux + Windows locally with cargo-zigbuild. macOS is CI-only (ADR 0005).
cross:
    rustup target add x86_64-unknown-linux-gnu x86_64-pc-windows-gnu
    cargo zigbuild --workspace --release --target x86_64-unknown-linux-gnu.2.17
    cargo zigbuild --workspace --release --target x86_64-pc-windows-gnu
    @echo "macOS: CI only — see docs/decisions/0005-ci-only-macos-releases.md"

# ----------------------------------------------------------------- tooling --

# Check the local toolchain
doctor:
    #!/usr/bin/env bash
    set -uo pipefail
    missing=0
    echo "required:"
    for t in git cargo rustc rustup just cargo-deny; do
      if command -v "$t" >/dev/null 2>&1; then printf '  ok       %s\n' "$t"; else printf '  MISSING  %s\n' "$t"; missing=1; fi
    done
    echo "needed later (cross builds, GitHub, Python bindings):"
    for t in gh cargo-zigbuild zig python3 maturin claude; do
      if command -v "$t" >/dev/null 2>&1; then printf '  ok       %s\n' "$t"; else printf '  missing  %s\n' "$t"; fi
    done
    if [ "$missing" -ne 0 ]; then
      echo; echo "Install hints:"
      echo "  cargo install just cargo-deny --locked"
      echo "  cargo install cargo-zigbuild --locked && pip install ziglang   # for 'just cross'"
      echo "  brew install gh                                              # GitHub CLI (macOS)"
      exit 1
    fi

# Regenerate Claude Code slash commands (/role-<name>) from docs/agents/
sync-agents:
    mkdir -p .claude/commands
    for f in docs/agents/*.md; do n="$(basename "$f" .md)"; { printf -- '---\ndescription: Start a session as the %s agent (docs/agents/%s.md)\n---\n\n' "$n" "$n"; cat "$f"; printf '\n\nAdditional instructions from the human (may be empty): $ARGUMENTS\n'; } > ".claude/commands/role-$n.md"; done
    @ls -1 .claude/commands
