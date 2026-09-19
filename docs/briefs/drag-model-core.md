# Task: {{SLUG}}
Role: {{ROLE}}   Phase: 0   Branch: {{ROLE}}/{{SLUG}}   Slot: numerics
blocks: drag-model-g7, point-mass-rk4   blocked-by: units-atmosphere

## Goal
Add a `DragModel` trait, the G1 standard drag table, and a first-class
custom Cd-vs-Mach table loader to `crates/ballistics`.

## In scope — you may edit only these
- `crates/ballistics/src/drag/**` (new module: `mod.rs`, `standard.rs`
  with G1 only, `custom.rs`)
- `crates/ballistics/src/lib.rs` (module wiring only)

## Out of scope — do not touch
- `crates/app`, `crates/ballistics-py`, `tools/`
- The G7 table — that is `drag-model-g7`, sequenced after this task
  because both land in `src/drag/standard.rs`
- `tests/fixtures/**`, `justfile`, `.github/**`

## Done when
- [ ] `trait DragModel { fn cd(&self, mach: Mach) -> f64; }` (or your
      considered equivalent), using the `Mach` newtype from
      `units-atmosphere`.
- [ ] G1 standard table implemented behind that trait, published source
      cited in a module-level doc comment (author, title, edition/year
      — this is a citation of the table values, not a derivation).
- [ ] Interpolation method (e.g. log-linear in Mach) documented where
      implemented, with the reason chosen over the alternative.
- [ ] Custom Cd-vs-Mach loader built now as a first-class citizen of
      the same trait, not bolted on later — pick a simple input format
      (e.g. two-column CSV `mach,cd`), document it in the loader's doc
      comment, and flag this scope decision in the PR per report §5.
- [ ] Unit tests: G1 table returns sane values at a few known Mach
      points; custom loader round-trips a small in-test table; loader
      rejects malformed input with a useful error, not a panic.
- [ ] `just verify` green.
- [ ] Handoff note written.

## Expected diff size
~280 lines. The G1 table's data is a meaningful fraction of this — if
you find yourself also drafting G7 to "save a PR," stop: that's the
next task, sequenced separately to keep this one under budget.

## Known constraints
- Report §5 — BC-as-form-factor's constant-across-velocity assumption is
  "the dominant Tier-1 error"; the trait must make a full Cd-vs-Mach
  table first-class, not a special case bolted onto a scalar BC.
- `docs/agents/ballistics.md` — every public function documents units;
  no `unsafe` without justification; deterministic (no `HashMap`
  iteration, no parallel float reduction in this path).
- A CSV-parsing crate beyond `std` for the loader needs an ADR in this
  same PR (`CLAUDE.md`); a hand-rolled two-column parser may be simpler
  — your call, flag which you chose.
