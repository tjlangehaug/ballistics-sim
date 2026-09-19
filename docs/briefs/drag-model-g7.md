# Task: {{SLUG}}
Role: {{ROLE}}   Phase: 0   Branch: {{ROLE}}/{{SLUG}}   Slot: numerics
blocks: point-mass-rk4   blocked-by: drag-model-core

## Goal
Add the G7 standard drag table to the existing `DragModel` trait and
interpolation machinery from `drag-model-core`.

## In scope — you may edit only these
- `crates/ballistics/src/drag/standard.rs` (extend; G1 must remain
  bit-identical — this task adds, it does not refactor)

## Out of scope — do not touch
- `crates/ballistics/src/drag/custom.rs`, `mod.rs` (unless a genuine
  trait signature bug is discovered — if so, stop and flag it rather
  than fixing it silently, since `drag-model-core` already merged)
- Anything outside `crates/ballistics/src/drag/`
- `crates/app`, `crates/ballistics-py`, `tools/`, `tests/fixtures/**`,
  `justfile`, `.github/**`

## Done when
- [ ] G7 standard table implemented behind the existing `DragModel`
      trait, published source cited in a doc comment (same standard as
      G1's citation in `drag-model-core`).
- [ ] Existing G1 tests are unaffected and still pass unmodified —
      this task adds a table, it does not touch G1's.
- [ ] A projectile can select G1 or G7 (or a custom table) at
      construction/solve time — confirm the existing trait-based design
      already supports this before writing new plumbing; if it doesn't,
      that's a real gap in `drag-model-core` to flag, not silently patch
      around.
- [ ] Unit tests: G7 table returns sane values at a few known Mach
      points, mirroring the G1 test shape.
- [ ] `just verify` green.
- [ ] Handoff note written.

## Expected diff size
~130 lines — one more data table plus tests reusing infrastructure
`drag-model-core` already built. If it's not small, the likely cause is
that trait not actually being reusable as designed; say so.

## Known constraints
- Same citation and unit-documentation bar as `drag-model-core`
  (`docs/agents/ballistics.md`).
- No numerical-output change to G1 — if `just regress` exists by the
  time this lands (it won't yet; `validation-harness` is still ahead of
  this in sequence) any G1 fixture must remain bit-identical.
