# Task: {{SLUG}}
Role: {{ROLE}}   Phase: 0   Branch: {{ROLE}}/{{SLUG}}   Slot: infra
blocks: Phase 1 start   blocked-by: validation-harness, py-bindings

## Goal
Close out Phase 0: reconcile the plan against what actually happened,
audit provenance and citations, and update `docs/VALIDATION.md` to
reflect that point-mass external ballistics is now validated.

## In scope — you may edit only these
- `docs/plan/phase-0-validation.md` (reconciliation, not rewriting
  history — annotate drift, don't erase it)
- `docs/plan/00-roadmap.md` (Phase 0 status, exit-criterion checkbox)
- `docs/VALIDATION.md`
- `docs/CHANGELOG.md`
- `docs/decisions/**` — only to flag a contradiction as a new
  superseding ADR if code and an existing ADR disagree; never to
  silently edit an existing ADR's Decision

## Out of scope — do not touch
- `crates/**` — if you find a defect (uncited constant, undocumented
  unit), file it as an issue / note for the owning role; do not fix it
  yourself (`docs/agents/docs.md`).

## Done when
- [ ] `docs/plan/phase-0-validation.md`: every task's checkbox matches
      reality; any task completed differently than its brief described
      is annotated with what actually happened and why.
- [ ] Provenance audit: every record in
      `docs/provenance/trajectory-runs.jsonl` carries every field
      `docs/provenance/README.md` requires. Missing fields are flagged,
      not silently backfilled.
- [ ] Citation audit: the G1 and G7 tables (`drag-model-core`,
      `drag-model-g7`) and every published fixture
      (`validation-harness`) cite a real source in the file itself. Any
      gap is filed as an issue against the owning role, not fixed here.
- [ ] `docs/VALIDATION.md` updated: point-mass (G1/G7/custom) moves
      from "Not yet validated" to validated against the specific
      fixture(s) landed, scoped per ADR 0008 (e.g. "validated at
      ranges/conditions where Coriolis is within stated tolerance").
      Terminal-ballistics disclaimer is unrelated and unchanged.
- [ ] `docs/CHANGELOG.md` has one line per merged Phase 0 PR (source of
      truth: `gh pr list --state merged`).
- [ ] Roadmap's Phase 0 row exit criterion checked off if genuinely met
      (one published reference trajectory reproduced within tolerance
      in CI); if not fully met, say so plainly rather than checking it.
- [ ] Handoff note written.

## Expected diff size
~200 lines, docs only.

## Known constraints
- `docs/agents/docs.md` — load-bearing role, not a tidying one; write
  for a reader six months from now with no memory.
- Honesty obligation on validation claims (`docs/VALIDATION.md`,
  `docs/agents/docs.md`) — never soften a claim; say plainly if a
  source is a single uncorroborated reference.
- Phase gate rule (`docs/plan/00-roadmap.md`) — this audit is one of
  four conditions before Phase 1 may start; the human still writes
  "What I learned," not you.
