# Phase 0 — Validation harness before anything visual

## Goal
Rust library crate: f64 point-mass integrator, G1/G7 tables, custom-drag-table
loader, independent atmosphere inputs, PyO3 bindings, and a regression suite
against published reference trajectories with a stated tolerance. No graphics.
If this phase does not match published data, nothing downstream matters.

## Exit criterion (roadmap)
One published reference trajectory reproduced within tolerance in CI.

## Sequencing
All numerics tasks touch `crates/ballistics` and must run strictly
sequentially — the parallelism rule (`AGENTS.md`) forbids two numerics
tasks at once. `validation-harness` is deliberately sequenced **after**
`point-mass-rk4`, not before it, even though `docs/agents/test.md`'s
general guidance is to write tests ahead of the feature: `just regress`
runs `crates/ballistics/tests/published_reference.rs` directly (no SKIP
branch once this phase lands), and `gate-test-integrity` fails on *any*
added `#[ignore]` in the diff, with no carve-out for the
"awaits <feature>" convention `test.md` recommends elsewhere. Writing
the harness against a solver that already exists avoids the conflict
entirely for this phase. **Flagging this as a standing contradiction
between `docs/agents/test.md` and the CI gate for the docs agent to
reconcile in a future close-out** — not resolving it here, since it's
broader than Phase 0.

`py-bindings` touches `crates/ballistics-py` + `tools/`, not
`crates/ballistics`, so it parallelises with `validation-harness` (slot:
infra vs numerics/test — 2 of the 3 allowed concurrent slots in use).

```
units-atmosphere -> drag-model-core -> drag-model-g7 -> point-mass-rk4 -> validation-harness -\
                                                                        -> py-bindings ---------+-> phase-0-closeout
```

## Tasks
- [x] `pr-gates` — ci — `pr.yml` + diff-size / test-integrity / provenance
      gates. **Done** (merged; `docs/handoff/2026-09-15-pr-gates.md`).
- [ ] `units-atmosphere` — ballistics — unit newtypes (`Mach`, `Metres`,
      `Radians`, …); independent station pressure / temperature /
      humidity / altitude inputs, no silent ICAO defaults.
      [slot: numerics, ~220 ln] blocked-by: none
- [ ] `drag-model-core` — ballistics — `DragModel` trait, G1 standard
      table, custom Cd-vs-Mach table loader (first-class, per report §5,
      not bolted on after G1/G7).
      [slot: numerics, ~280 ln] blocked-by: `units-atmosphere`
- [ ] `drag-model-g7` — ballistics — G7 standard table via the existing
      trait/interpolation from `drag-model-core`.
      [slot: numerics, ~130 ln] blocked-by: `drag-model-core`
- [ ] `point-mass-rk4` — ballistics — f64 RK4 point-mass integrator;
      provenance record per run (schema: `docs/provenance/README.md`).
      No Earth-frame Coriolis/Eötvös in Phase 0 — deferred to Phase 1
      per ADR 0008.
      [slot: numerics, ~280 ln] blocked-by: `drag-model-g7`
- [ ] `validation-harness` — test — fixture format per ADR 0007; ≥1
      cited published fixture; `crates/ballistics/tests/published_reference.rs`
      calling the now-existing `solve()`; **removes the SKIP branch from
      `just regress`**.
      [slot: numerics/test, ~250 ln] blocked-by: `point-mass-rk4`
- [ ] `py-bindings` — pybind — PyO3/maturin `solve()` + `solve_batch()`;
      bit-identical Python vs Rust test; `.pyi` stub.
      [slot: infra, ~300 ln] blocked-by: `point-mass-rk4` · parallel with:
      `validation-harness`
- [ ] `phase-0-closeout` — docs — plan reconciliation, provenance +
      citation audit, `VALIDATION.md` update.
      [slot: infra, ~200 ln] blocked-by: `validation-harness`, `py-bindings`

Briefs for all unchecked tasks are in `docs/briefs/`.

## HUMAN DECISIONS REQUIRED
- [x] **ADR 0008** — Earth-frame Coriolis/Eötvös: deferred to Phase 1.
      "Leave Coriolis effect for a future phase. I want a good basic
      physics model at this point." (2026-09-19)
- [ ] Which published external-ballistics source(s) to use for the first
      fixture(s) (must be citable). Candidates to consider — none vetted,
      offered only as a starting point for your review: McCoy, *Modern
      Exterior Ballistics* (worked examples with stated conditions);
      manufacturer-published Doppler-radar-derived trajectory tables
      (e.g. Hornady, Sierra) where the underlying conditions are fully
      stated; Litz, *Applied Ballistics* validation tables; a
      military TM (e.g. TM 43-0001-27 family) firing table, if a full
      G-function and atmosphere are stated. The `validation-harness`
      task's executor should propose specific sources with citations in
      the PR for your approval, per `docs/agents/test.md`.
- [ ] Tolerance per quantity (drop, drift, velocity, TOF), each with a
      physical rationale — proposed by whoever executes
      `validation-harness`, approved by you in that PR. Not decided here.

## Already settled — do not re-litigate
Fixture file format: ADR 0007 (TOML, one file per fixture). Coriolis/
Eötvös deferred to Phase 1: ADR 0008. f64 CPU solver, wgpu, two-tier
terminal ballistics, CI-only macOS releases: ADRs 0001–0005 (report
§§2,5,6,8).

## What I learned
<human writes this at phase close>
