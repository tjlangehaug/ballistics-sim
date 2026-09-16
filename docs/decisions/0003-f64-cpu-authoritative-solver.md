# 0003 — f64 on CPU is the only authoritative simulation path
Status: Accepted
Date: 2026-09-15

## Context
WGSL has no f64 and Metal has never exposed double, so portable GPU compute is f32 only. Trajectories must match published firing data. (Report §1, §7.1, §9.1)

## Options
1. **CPU f64 solver, GPU display-only** — deterministic, precise; trajectory is microseconds on one core.
2. **GPU with compensated/double-single arithmetic** — costly, fragile, driver-variant.

## Decision
`crates/ballistics` is f64, CPU-only, zero graphics dependencies. GPU results never feed authoritative state. Solver runs on its own clock, decoupled from the engine tick.

## Consequences
Rendering converts f64→f32 at one named boundary using local-origin coordinates.

## Revisit if
Never, for the authoritative path. (Verify the WGSL f64 claim against the current spec once — report Table 6.)
