# Handoff: units-atmosphere (newtypes) — 2026-09-19
Role: ballistics   Branch: ballistics/units-atmosphere   PR: (opening now, split in two)

## What changed, and why
- `crates/ballistics/src/units.rs` (new): unit newtypes `Meters`,
  `Radians`, `MetersPerSecond`, `Mach`, `Kelvin`, `Pascals`,
  `RelativeHumidity`, `KilogramsPerCubicMeter`. Each documents its unit
  in a doc comment, has a minimal `new`/`value` pair, and validates its
  invariant at construction (positive temperature/pressure/density, a
  `[0.0, 1.0]` fraction for humidity) rather than accepting a value
  that can't physically occur.
- `crates/ballistics/src/lib.rs`: added `pub mod units;` — wiring only.
- Why: `docs/briefs/units-atmosphere.md` (Phase 0, first unblocked
  task).

## Deliberately NOT done
- No arithmetic trait impls on the newtypes (`impl Add for Meters`,
  etc.) — the brief flagged this as the likely overrun risk, and
  nothing in this task's requirements needs it.
- No `speed_of_sound`/velocity-to-`Mach` conversion here, even though
  it's atmosphere-dependent — not this task's scope; flagged for
  whoever picks up `drag-model-core` or `point-mass-rk4` in this PR's
  sibling handoff note (see `2026-09-19-units-atmosphere-density.md`).
- **Split into two PRs on realizing mid-task the combined diff would
  exceed 400 lines** (`CLAUDE.md` diff budget): this PR carries the
  newtypes; a sibling PR (branch `ballistics/units-atmosphere-density`,
  based on this one) carries `Atmosphere` itself and `air_density()`.
  Both together satisfy `docs/briefs/units-atmosphere.md`'s full "Done
  when" list — this PR alone does not yet (no `Atmosphere` type).

## Surprises a future session must know
- `clippy::float_cmp` (workspace lint, `-D warnings`) fires on
  `assert_eq!` between two `f64`s even for an *exact* round-trip check
  (constructor stores the value unchanged, no arithmetic). Used
  `#[allow(clippy::float_cmp)]` with a comment on that one test
  function, scoped narrowly — not a blanket allow.

## Exact next action
1. Review and merge this PR, then its sibling
   (`ballistics/units-atmosphere-density`).
2. Next unblocked task after both land: `drag-model-core`.
