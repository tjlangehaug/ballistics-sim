# Task: {{SLUG}}
Role: {{ROLE}}   Phase: 0   Branch: {{ROLE}}/{{SLUG}}   Slot: numerics
blocks: drag-model-core   blocked-by: none

## Goal
Add unit newtypes and an independent-inputs atmosphere model to
`crates/ballistics`, with no silently-derived or ICAO-default values.

## In scope — you may edit only these
- `crates/ballistics/src/**` (new modules, e.g. `units.rs`, `atmosphere.rs`)
- `crates/ballistics/Cargo.toml` (only if a dependency is unavoidable —
  see Known constraints)

## Out of scope — do not touch
- `crates/app`, `crates/ballistics-py`, `tools/`
- `tests/fixtures/**` (not your task; `validation-harness` owns fixtures)
- `justfile`, `.github/**`

## Done when
- [ ] Newtypes exist for at least: `Mach`, `Metres` (or your considered
      naming), `Radians`, `MetresPerSecond` — wherever a bare `f64`
      would otherwise be ambiguous. Every public item's doc comment
      states its unit.
- [ ] `Atmosphere` (or equivalent) takes station pressure, temperature,
      humidity, and altitude as independent fields — never derives one
      from another, never silently substitutes an ICAO standard value.
      If a caller omits a field, that is a compile error or an explicit
      `Option`, not a hidden default.
- [ ] Any derived quantity (e.g. air density) is computed via a formula
      whose derivation is a doc comment with units at each step, traced
      to a *stated* reference condition — never a copied constant.
- [ ] Unit tests cover the derivation at at least one known reference
      condition (e.g. ICAO standard sea level, used here only as a
      *check value*, not as a silent default anywhere in the API).
- [ ] `just verify` green.
- [ ] Handoff note written.

## Expected diff size
~220 lines. Most likely overrun: also wiring in unit *arithmetic* (e.g.
`impl Add for Metres`) — if so, cut it and flag as a scope decision.

## Known constraints
- ADR 0003 — f64, CPU-only, no graphics deps, forever
  (`docs/agents/ballistics.md`).
- Report §5 — atmosphere inputs independent (pressure, temperature,
  humidity, altitude); no silent ICAO defaults.
- `CLAUDE.md` — never copy a physical constant; derive it from a stated
  reference condition, derivation as a doc comment with units at each
  step (e.g. the specific gas constant for dry air, if used).
- A derivation needing a crate beyond `std` (e.g. vapour-pressure calc)
  requires an ADR in this same PR — prefer a direct closed-form
  implementation to avoid that overhead where reasonable.
