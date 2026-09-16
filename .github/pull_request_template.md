## Task
Brief: `.task/BRIEF.md` — <one line>
ADR: <link, or "none needed">
Phase: <from docs/plan/00-roadmap.md>

## What changed
<3-6 bullets, reviewer-oriented, not a commit log>

## What I deliberately did not do
<scope boundaries. Empty is a red flag.>

## Numerical impact
- [ ] No effect on trajectory output
- [ ] Affects output — before/after table below, `just regress` attached

## Verification
- [ ] `just verify` passes locally
- [ ] New behaviour has a test that fails without this change
- [ ] No test was modified, skipped, or had its tolerance changed
- [ ] Diff under 400 lines excl. tests/fixtures/lockfiles

## Reviewer attention
<where you are least confident. Be specific. "All of it" is not useful.>
