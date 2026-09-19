# Task: {{SLUG}}
Role: {{ROLE}}   Phase: 0   Branch: {{ROLE}}/{{SLUG}}   Slot: numerics
blocks: validation-harness, py-bindings   blocked-by: drag-model-g7

## Goal
Implement the f64 RK4 point-mass integrator with a per-run provenance
record, producing `crates/ballistics`'s first end-to-end `solve()`.

## Scope note
Per ADR 0008 (Accepted): no Earth-frame Coriolis/Eötvös term this task.
`Projectile`/`solve()` take no firing-point latitude/azimuth in Phase 0
— deferred to Phase 1. Keep the model to gravity + drag; don't add a
placeholder hook for Coriolis "for later" (`CLAUDE.md` — don't design
for hypothetical future requirements).

## In scope — you may edit only these
- `crates/ballistics/src/**` (new integrator/solver module; `lib.rs`
  wiring for the public `solve()` entry point)

## Out of scope — do not touch
- `crates/ballistics/src/drag/**`, `units.rs`, `atmosphere.rs` (use
  them; don't redesign them — if they're missing something you need,
  flag it in the PR rather than expanding their scope here)
- `crates/app`, `crates/ballistics-py`, `tools/`, `tests/fixtures/**`,
  `justfile`, `.github/**`
- Writing to `docs/provenance/trajectory-runs.jsonl` — return a
  provenance record as data; appending it to the ledger is the
  caller's job, not this crate's. Keep it free of file I/O.

## Done when
- [ ] `solve()` (or your considered name) takes a projectile, an
      atmosphere, and a `DragModel`, and returns a trajectory (state at
      each step or a queryable path) using fixed-step RK4.
- [ ] Single-threaded, deterministic: same inputs -> bit-identical
      output, always.
- [ ] Returns a plain provenance struct covering every field in
      `docs/provenance/README.md` (solver version, `"rk4"`, step size,
      drag-table identity, atmosphere inputs, thread count 1, target
      triple, Coriolis/Eötvös noted absent per ADR 0008). If JSON
      serialization needs a new dependency, push it to the caller
      instead of adding one here.
- [ ] Property tests per `docs/agents/test.md` where they apply to a
      bare point-mass integrator: e.g. speed monotonically decreasing
      under drag alone, symmetric no-wind trajectory.
- [ ] `just verify` green. (`just regress` will still short-circuit via
      its SKIP branch — that's `validation-harness`'s job to remove,
      not yours.)
- [ ] Handoff note written.

## Expected diff size
~280 lines.

## Known constraints
- ADR 0008 (Coriolis/Eötvös out of scope this phase), ADR 0003 (f64,
  CPU-only).
- Report §5 — solver decoupled from engine tick (compute whole flight
  at trigger pull; renderer interpolates later) — keep `solve()`
  computing a full trajectory up front, not a step-by-step tick API.
- Report §9 — provenance fields listed above; determinism (no
  `HashMap` iteration; stay single-threaded here).
- A change to `drag/**`, `units.rs`, or `atmosphere.rs` turning out to
  be genuinely necessary is a scope question for the human, not a
  silent expansion of this brief.
