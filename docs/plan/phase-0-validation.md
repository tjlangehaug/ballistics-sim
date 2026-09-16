# Phase 0 — Validation harness before anything visual

> **DRAFT.** Seeded from the main report §11. The Architect must decompose,
> size, sequence, and write a brief in `docs/briefs/<slug>.md` for every task
> before any worker starts.

## Goal
Rust library crate: f64 point-mass integrator, G1/G7 tables, custom-drag-table
loader, independent atmosphere inputs, PyO3 bindings, and a regression suite
against published reference trajectories with a stated tolerance. No graphics.
If this phase does not match published data, nothing downstream matters.

## Tasks (draft)
- [ ] `pr-gates` — ci — `pr.yml` + diff-size / test-integrity / provenance gates [slot: infra]
- [ ] `validation-harness` — test — fixture format spec, ≥1 cited published fixture, `crates/ballistics/tests/published_reference.rs`; **remove the SKIP branch from `just regress`** [slot: numerics/tests]
- [ ] `units-atmosphere` — ballistics — unit newtypes; independent station pressure / temperature / humidity / altitude inputs [slot: numerics]
- [ ] `drag-tables` — ballistics — `DragModel` trait; G1 + G7 tables; custom Cd-vs-Mach loader (first-class, not bolted on) [slot: numerics]
- [ ] `point-mass-rk4` — ballistics — f64 RK4 point-mass integrator; provenance record per run [slot: numerics]
- [ ] `py-bindings` — pybind — PyO3/maturin `solve()` + `solve_batch()`; bit-identical Python vs Rust test [slot: infra]
- [ ] `phase-0-closeout` — docs — plan reconciliation, provenance + citation audit, `VALIDATION.md` [slot: infra]

## HUMAN DECISIONS REQUIRED
- [ ] Which published external-ballistics sources to use for fixtures (must be citable)
- [ ] Tolerances per quantity (drop, drift, velocity, TOF) with physical rationale
- [ ] Fixture file format (ADR)
- [ ] Earth frame / Coriolis in Phase 0 or deferred to Phase 1?
