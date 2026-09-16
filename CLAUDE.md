# Project contract
Ballistics simulator: validated external ballistics + 3D visualisation.
Cross-platform (macOS / Linux / Windows). Solo developer. Rust.

## Start of every session — do this first, before anything else
1. Read `docs/plan/00-roadmap.md`. Identify the CURRENT PHASE.
2. Read the current phase file in `docs/plan/`.
3. Read the newest 2 files in `docs/handoff/`.
4. Read your task brief at `.task/BRIEF.md` in this worktree.
5. State back to me, in under 10 lines: the phase, your task, the files
   you expect to touch, and your expected diff size. Then WAIT for my go.

Do not write code before step 5 is acknowledged.

## Where things live
- Role prompts: `docs/agents/<role>.md` (index: `AGENTS.md`)
- Settled decisions: `docs/decisions/` — do not re-litigate them
- Technical constraints by section (§): `docs/reference/technical-report-summary.md`
- What is and is not validated: `docs/VALIDATION.md`

## Hard rules
- NEVER commit to `main`. You are on a task branch; verify with
  `git branch --show-current` before your first commit.
- NEVER run a command that is not a `just` target. If you need a new
  command, add a `just` target for it in this same PR.
- NEVER modify, weaken, skip, or delete a test to make it pass. If a test
  is genuinely wrong, stop and tell me; do not fix it yourself.
- NEVER change numerical tolerances in `tests/`. Tolerances are physics
  decisions and they are mine.
- NEVER touch a crate outside your task's declared scope. Parallel agents
  are working in other crates.
- NEVER add a dependency without an ADR in the same PR.
- STOP and ask if the task requires a decision not already covered by an
  ADR in `docs/decisions/`.

## Diff budget
Target under 400 changed lines excluding tests, fixtures, lockfiles.
CI fails above that. If you realise mid-task you will exceed it: stop,
commit what is coherent, open the PR, and write the remainder as a
follow-up task brief in `docs/briefs/`. A split PR is a success, not a failure.

## Numerical work
- `crates/ballistics` is f64, CPU-only, no graphics dependencies, ever.
- GPU results never feed back into authoritative simulation state.
- Any change touching trajectory output must run `just regress` and paste
  the before/after table into the PR body.
- Never copy a physical constant from a source. Derive it from a stated
  reference condition and document the derivation in a comment.

## Commands (the complete set)
    just verify        # fmt + clippy + test — run before every commit
    just test          # unit + integration
    just regress       # numerical regression vs tests/fixtures/published
    just ci-local      # everything the PR pipeline runs
    just deny          # licence + advisory check
    just cross         # build Linux + Windows targets locally
    just bench         # criterion benchmarks
    just doctor        # check the local toolchain
    just task SLUG ROLE  # create branch + worktree + brief   (human only)
    just land SLUG       # tear down worktree after merge     (human only)
    just sync-agents   # regenerate .claude/commands from docs/agents

## End of every session — non-negotiable
1. `just verify` must pass.
2. Update the checkbox for your task in the phase file.
3. Write `docs/handoff/YYYY-MM-DD-<slug>.md` (template: `docs/templates/HANDOFF.md`):
   - what changed, and why
   - what is deliberately NOT done
   - anything surprising a future session must know
   - the exact next action
4. Append one line to `docs/CHANGELOG.md`.
5. Open the PR using the template. Fill every section honestly.

If you cannot complete step 1, say so explicitly in the PR body and mark
the PR as draft. Never describe work as complete when CI is red.
