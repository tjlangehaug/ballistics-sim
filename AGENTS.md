# Agent roles

Every agent first obeys `CLAUDE.md`. Pick ONE role per session, ONE task per
session, ONE worktree per task. Start a session by running the matching slash
command (e.g. `/role-ballistics`) or pasting `docs/agents/<file>` as the first
message.

| # | Role | Prompt | Slash command | Branch prefix | Slot | Writes to |
|---|------|--------|---------------|---------------|------|-----------|
| 1 | Architect / Planner | `docs/agents/architect.md` | `/role-architect` | `arch/` | any | `docs/plan/`, `docs/decisions/`, `docs/briefs/` — never `crates/` |
| 2 | Ballistics Numerics | `docs/agents/ballistics.md` | `/role-ballistics` | `ballistics/` | 1 numerics | `crates/ballistics` |
| 3 | Test & Validation | `docs/agents/test.md` | `/role-test` | `test/` | 1 or 3 | `tests/`, fixtures |
| 4 | Code Reviewer | `docs/agents/reviewer.md` | `/role-reviewer` | none (main checkout) | — | review comment only |
| 5 | CI/CD & Build | `docs/agents/ci.md` | `/role-ci` | `ci/` | 3 infra | `.github/`, `justfile` |
| 6 | Physics-Engine Integration | `docs/agents/physics.md` | `/role-physics` | `physics/` | 2 shell | `crates/app` |
| 7 | Graphics & Rendering | `docs/agents/render.md` | `/role-render` | `render/` | 2 shell | `crates/app` |
| 8 | Python Bindings | `docs/agents/pybind.md` | `/role-pybind` | `pybind/` | 3 infra | `crates/ballistics-py`, `tools/` |
| 9 | Performance Profiler | `docs/agents/perf.md` | `/role-perf` | `perf/` | per brief | per brief |
| 10 | Documentation & Provenance | `docs/agents/docs.md` | `/role-docs` | `docs/` | 3 infra | `docs/` |
| 11 | Release & Packaging | `docs/agents/release.md` | `/role-release` | `release/` | 3 infra | `.github/`, `justfile`, `docs/` |

## Parallelism rule
At most 3 concurrent tasks, each in a different slot:
- **Slot 1 — numerics:** `crates/ballistics`. Never two numerics tasks at once.
- **Slot 2 — app shell:** `crates/app` (physics, render, UI).
- **Slot 3 — infrastructure:** CI, docs, bindings, packaging.

Two concurrent tasks may never touch the same crate. If you want a 4th agent,
your review queue is the bottleneck — don't.
