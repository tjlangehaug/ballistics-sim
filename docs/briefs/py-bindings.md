# Task: {{SLUG}}
Role: {{ROLE}}   Phase: 0   Branch: {{ROLE}}/{{SLUG}}   Slot: infra
blocks: phase-0-closeout   blocked-by: point-mass-rk4
parallel with: validation-harness (different crates, different slots)

## Goal
Expose `crates/ballistics`'s `solve()` through PyO3/maturin bindings in
`crates/ballistics-py`, proving Python and Rust produce bit-identical
output for the same input.

## In scope — you may edit only these
- `crates/ballistics-py/**`
- `tools/**`
- `Cargo.toml` (root) — adding `crates/ballistics-py` as a workspace
  member; the existing comment there says this crate joins "in Phases
  0/2 when it gets dependencies" — this task is that moment
- `justfile` — add a `py-test` recipe only (per `CLAUDE.md`, any new
  command needs a `just` target in the same PR); do not touch other
  recipes

## Out of scope — do not touch
- `crates/ballistics/src/**` — bind the existing API, don't change it;
  an API-change need goes in your handoff note, not a same-PR fix
  (`docs/agents/pybind.md`).
- `crates/app`, `.github/**` (wiring `py-test` into CI is a follow-up
  CI-role task)

## Done when
- [ ] `crates/ballistics-py` exposes `Projectile`, `Atmosphere`, a
      single-shot `solve()`, and a `solve_batch()` keeping the loop in
      Rust (not a Python loop over single-shot `solve()`).
- [ ] `solve_batch()` returns numpy arrays, not lists of tuples.
- [ ] The GIL is released around the actual solve call(s) so batches
      parallelize.
- [ ] Units and naming in every Python-facing signature/docstring match
      the Rust crate's conventions exactly.
- [ ] Rust panics never cross the FFI boundary uncaught — they become
      Python exceptions with a useful message.
- [ ] `ballistics.pyi` type stub, covering the full exposed surface.
- [ ] A test asserting Python's `solve()` and a Rust-side call with the
      same inputs produce bit-identical output — proves the
      single-source-of-truth property (`docs/agents/pybind.md`).
- [ ] `just py-test` target added and green.
- [ ] Handoff note written, including any API-change request against
      `crates/ballistics` if one came up.

## Expected diff size
~300 lines. PyO3 boilerplate (module init, error conversion, numpy
glue) is a fair share of this — trim there first, not the
bit-identical test, if it's tight.

## Known constraints
- `docs/agents/pybind.md` — one implementation, two consumers; Python
  must never reimplement physics for convenience.
- `pyo3`, `numpy` (Rust crate), and `maturin` are new dependencies,
  each needing an ADR in this same PR (`CLAUDE.md`). Expected; write it.
- Report §9 — solver library crate first, app second; the same compiled
  solver serves validation, sweeps, plotting, and future surrogate
  fitting.
