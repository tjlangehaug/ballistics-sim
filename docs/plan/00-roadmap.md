# Roadmap

**CURRENT PHASE: Setup** (then Phase 0)

## Phase gate rule (Appendix A §A.8.1)
A phase closes only when: every checkbox in its file is ticked, nightly gates
are green, the Documentation agent's close-out audit is merged, and the human
has written a "What I learned" paragraph below. Only then may a task from the
next phase start. No exceptions — this is the convergence mechanism.

## Phases
| Phase | Name | File | Exit criterion | Status |
|---|---|---|---|---|
| Setup | Contract + gates (≤ 1 day) | this file | Checklist below complete | in progress |
| 0 | Validation harness before anything visual | `phase-0-validation.md` | One published reference trajectory reproduced within tolerance in CI | not started |
| 1 | Fidelity ladder (MPMM → 6-DOF) | `phase-1-fidelity.md` | Each tier validated; lower tiers cross-check higher | not started |
| 2 | Visualisation shell | `phase-2-visualisation.md` | Bevy app renders a solved trajectory; 3-OS CI green | not started |
| 3 | Terminal ballistics, Tier A (offline) | `phase-3-terminal-offline.md` | Versioned penetration/fragmentation response database | not started |
| 4 | Terminal ballistics, Tier B (runtime) | `phase-4-terminal-runtime.md` | Surrogate lookup + GPU fragments in app | not started |
| 5 | Product surface | `phase-5-product.md` | v0.1 pre-release with provenance + VALIDATION.md | not started |

## Setup checklist (Appendix A §A.10)
- [ ] Public GitHub repo created and pushed (`main`)
- [ ] Branch protection on `main`: require PR, require `pr.yml` check (add once it exists), up-to-date branches, linear history, auto-delete merged branches. Do NOT require approvals.
- [ ] Squash-merge only enabled in repo settings
- [ ] `needs-cross` and `large-diff-approved` labels created
- [ ] Human decision: Rust edition/toolchain pin confirmed (`rust-toolchain.toml`)
- [x] First PR through the loop: CI agent — `pr.yml` + three custom gates (task `pr-gates`)
- [x] Architect session: Phase 0 decomposed into sub-400-line briefs in `docs/briefs/`
- [ ] Test agent: validation harness + ≥1 published fixture
- [ ] Ballistics agent: point-mass integrator reproduces that fixture in CI

## What I learned
### Setup
<human writes this at phase close>
