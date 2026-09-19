# Handoff: units-atmosphere (Atmosphere + air_density) — 2026-09-19
Role: ballistics   Branch: ballistics/units-atmosphere-density   PR: (opening now)

## What changed, and why
- `crates/ballistics/src/atmosphere.rs` (new): `Atmosphere` with four
  independent fields (station pressure, temperature, relative
  humidity, altitude) — no `Default`, no field derives another, no
  silent ICAO substitution. `Atmosphere::icao_standard_sea_level()` is
  a named, explicit reference point, not a default anything applies
  automatically.
- `air_density()`, the derived quantity `docs/briefs/units-atmosphere.md`
  asked for. Every constant it uses is derived from a stated source
  rather than pasted from memory (`CLAUDE.md` "Numerical work"): the
  specific gas constants for dry air and water vapor are computed from
  the molar gas constant (exact, 2019 SI redefinition) divided by each
  substance's molar mass (U.S. Standard Atmosphere, 1976 for dry air;
  IUPAC standard atomic weights for water), and saturation vapor
  pressure uses Tetens' equation (Tetens, 1930) with its published
  coefficients. I used a web search this session to verify the molar
  mass of dry air and cross-check the Tetens coefficients before
  writing any of this into the crate, rather than trusting recall of a
  physical constant.
- `crates/ballistics/src/lib.rs`: added `pub mod atmosphere;` — wiring
  only.
- Why: `docs/briefs/units-atmosphere.md`, second half (this PR is
  stacked on `ballistics/units-atmosphere`, #5, which has the
  newtypes).

## Deliberately NOT done
- `air_density()` incorporates humidity via the full moist-air
  (Dalton's law) formula rather than a dry-air-only approximation —
  this is a scope addition beyond the brief's literal minimum (which
  only needed "a derived quantity"), made because leaving
  `relative_humidity` as an accepted-but-unused field felt worse than
  doing the extra, well-cited derivation. **Flagging this as the
  deliberate scope decision it is**, per `docs/agents/ballistics.md`'s
  convention — accept it or ask me to trim to dry-air-only.
- No `Cargo.toml` change — no dependency was needed for any of this.
- No `speed_of_sound`/velocity-to-`Mach` conversion — still flagged
  (see #5's handoff) for whoever picks up `drag-model-core` or
  `point-mass-rk4`; it belongs with whichever of those first needs to
  turn a velocity into a `Mach` number.

## Surprises a future session must know
- The doc comment on `air_density()` is long (it's the full derivation
  chain — molar gas constant, two molar masses, the Tetens formula,
  and the check-value note) — this is what drove this task over the
  400-line budget when combined with the newtypes PR, not the
  implementation itself, which is ~25 lines.
- Verified numerically during implementation, not just asserted: at
  `Atmosphere::icao_standard_sea_level()` (0% humidity), the derivation
  evaluates to ~1.2249 kg/m^3 against the ICAO Standard Atmosphere's
  published 1.225 kg/m^3 — a 0.008% difference from rounding the molar
  mass of dry air to 6 significant figures, well inside the test's
  0.001 kg/m^3 tolerance.

## Exact next action
1. Review and merge this PR (after #5, its base).
2. `just regress` still shows its SKIP branch — that's `validation-harness`'s
   job, sequenced after `point-mass-rk4`, not this task's.
3. Next unblocked task: `drag-model-core`.
