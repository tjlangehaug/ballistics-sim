# 0008 — Earth-frame Coriolis/Eötvös in Phase 0 point-mass
Status: Accepted
Date: 2026-09-19

## Context
Report §5 lists Coriolis and Eötvös effects in an Earth-fixed frame as
something to "budget for," alongside MV temperature coefficient and
rifle cant — but it assigns them to the general external-ballistics
section, not explicitly to a tier in the fidelity ladder (Tier 1 point
mass + G-model; Tier 2 modified point mass with yaw of repose, spin
drift, Magnus; Tier 3 full 6-DOF). Phase 0's stated goal
(`docs/plan/phase-0-validation.md`) is proving the validation harness
against a published reference trajectory with the simplest correct
solver, not maximum fidelity. Including Coriolis changes the Tier-1
solver's input surface (needs firing-point latitude and azimuth) and
its validation fixtures (a published trajectory must either state those
inputs or be far enough downrange/short enough flight time that
Coriolis is negligible relative to the fixture's tolerance).

## Options
1. **Include Coriolis/Eötvös in Phase 0's point-mass integrator.**
   Case: matches real long-range use (Coriolis drift is non-negligible
   past ~600–800 m for typical small-arms trajectories) and avoids a
   breaking API change to `Projectile`/`solve()` later when Phase 1
   needs it anyway. Cost: adds latitude + azimuth (+ optionally firing
   direction convention) to every Tier-1 call; complicates fixture
   selection, since a published source must supply those inputs or the
   comparison silently assumes zero Coriolis; delays the "prove the
   harness works" milestone with a term that has nothing to do with
   drag-model validation, which is what Phase 0 actually exists to test.
2. **Defer to Phase 1 (or a flagged Tier-1.5), zero Coriolis in Phase 0.**
   Case: keeps Phase 0's solver surface to exactly what its own exit
   criterion needs — matching a published drag-model trajectory — and
   most published short-to-mid-range reference tables (the likely
   Phase-0 fixture candidates) either don't report Coriolis separately
   or are short-range enough that it's within stated tolerance to omit.
   Cost: the point-mass API will need a non-trivial extension in Phase 1
   (new inputs, new fixture requirements) rather than being fully settled
   now.

## Decision
Option 2 — defer to Phase 1. Human decision, 2026-09-19: "Leave Coriolis
effect for a future phase. I want a good basic physics model at this
point." Phase 0's point-mass integrator omits Earth-frame Coriolis and
Eötvös terms entirely; `Projectile`/`solve()` take no latitude/azimuth
input in this phase.

## Consequences
- Easier: `point-mass-rk4`'s input surface stays exactly what Phase 0's
  own exit criterion needs (drag-model + atmosphere validation against a
  published trajectory); fixture selection is not constrained to sources
  that state firing-point latitude/azimuth.
- Harder: Phase 1 will need a breaking (additive) change to
  `Projectile`/`solve()` to add latitude + azimuth when Coriolis is
  introduced there, rather than the surface being settled now.
- The omission is recorded as a solver-configuration fact in every
  provenance record (`docs/provenance/README.md`) and in
  `docs/VALIDATION.md`'s description of what Tier 1 models — "validated
  against published data at conditions where Coriolis is within stated
  tolerance," not an unqualified "validated."

## Revisit if
A candidate published fixture for Phase 0 turns out to require Coriolis
to match within any defensible tolerance (i.e. Option 2 becomes
unworkable because no Coriolis-free fixture exists at citable quality).
