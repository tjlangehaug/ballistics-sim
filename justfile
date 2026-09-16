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
