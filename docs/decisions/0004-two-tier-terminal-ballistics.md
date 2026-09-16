# 0004 — Two-tier terminal ballistics
Status: Accepted
Date: 2026-09-15

## Context
FEM/MPM penetration with fragmentation cannot run in an interactive frame budget; the good codes are batch tools. Open validated terminal data is largely unavailable. (Report §6)

## Options
1. **Tier A offline MPM/FDEM → Tier B runtime surrogate + GPU particles** — defensible, fast, incrementally improvable.
2. **Live GPU MPM** — existence proof (CRESSim-MPM) for bounded particle counts only.

## Decision
Two tiers. Live MPM only as a stretch goal on one showcase target.

## Consequences
CUDA is allowed in Tier A (offline, Linux workstation). The product must state terminal effects are physically principled, not empirically validated.

## Revisit if
Validated open terminal-ballistics test data becomes available.
