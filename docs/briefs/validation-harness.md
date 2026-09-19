# Task: {{SLUG}}
Role: {{ROLE}}   Phase: 0   Branch: {{ROLE}}/{{SLUG}}   Slot: numerics/test
blocks: phase-0-closeout   blocked-by: point-mass-rk4

## Goal
Build the published-fixture validation harness: fixture format per ADR
0007, at least one cited published fixture, and
`crates/ballistics/tests/published_reference.rs` exercising the
now-existing `solve()` — then remove `just regress`'s SKIP branch.

## In scope — you may edit only these
- `tests/fixtures/published/**` (new fixture files + a format README)
- `crates/ballistics/tests/published_reference.rs` (new)
- `crates/ballistics/Cargo.toml` (dev-dependency only, for fixture
  parsing — see Known constraints, this needs its own ADR in this PR)
- `justfile` — **narrowly**: only the `regress` recipe's SKIP branch
  (delete it). Do not touch any other recipe. `justfile` is normally
  the CI role's file (`docs/agents/ci.md`); this one edit is
  pre-authorized because the SKIP comment itself says "the harness PR
  must delete it."

## Out of scope — do not touch
- `crates/ballistics/src/**` — you call `solve()`, you don't change it.
  If the API doesn't support what a fixture needs, that's a request in
  your handoff note, not a same-PR fix.
- `crates/app`, `crates/ballistics-py`, `tools/`, `.github/**`

## Done when
- [ ] `tests/fixtures/published/README.md` states the ADR 0007 TOML
      format, the citation requirement, and how to add a fixture.
- [ ] At least one real fixture, TOML, per ADR 0007: projectile params,
      atmosphere inputs, an `[[range]]` array-of-tables of published
      drop/drift/velocity/TOF, a tolerance per column stated as a
      physical quantity with a rationale (not a bare float —
      `docs/agents/test.md`), and the published source cited in the
      file itself.
- [ ] Source and tolerance choices are proposed in the PR description
      for human approval, per the HUMAN DECISIONS in
      `docs/plan/phase-0-validation.md` — do not treat your first
      attempt as final; flag it explicitly as pending approval.
- [ ] `published_reference.rs` parses every fixture, runs `solve()`,
      and on failure prints a per-range expected/actual/delta/tolerance
      table, marking the first row that exceeds tolerance.
- [ ] No `#[ignore]` anywhere in this diff — `gate-test-integrity` fails
      on any added `#[ignore]`, no "awaits <feature>" exception. Since
      `point-mass-rk4` already landed the test should simply pass; a
      failure is a solver gap to flag, not to ignore.
- [ ] `regress`'s SKIP branch is gone; `just ci-local` green.
- [ ] Handoff note listing sources used and — per `docs/agents/test.md`
      — sources sought but not obtained.

## Expected diff size
~250 lines, excluding fixture data (fixtures are excluded from the
diff-size gate). Keep the `justfile` SKIP-branch removal to a few
lines; don't let it grow into touching other recipes.

## Known constraints
- ADR 0007 — fixture file format (TOML, one file per fixture).
- `docs/agents/test.md` — fixture discipline: cited source, physical
  tolerance rationale, never widen an existing tolerance, tolerance
  values are a human decision to approve, not yours to finalize.
- `CLAUDE.md` — never change a numerical tolerance in `tests/` once it
  exists; `gate-test-integrity` enforces this mechanically too.
- `serde` + `toml` (or equivalent) for parsing fixtures is a new
  dependency — needs its own ADR in this same PR (`CLAUDE.md`); this is
  expected, write it rather than working around the need for one.
