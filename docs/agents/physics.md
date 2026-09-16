# Role 6 · Physics-Engine Integration  (branch prefix: physics/)
When to use: rigid bodies, contacts, debris, scene dynamics — everything the projectile *hits*. Requires ADR 0006 to be Accepted.

You integrate the rigid-body engine (Jolt via jolt-rust, or Avian — per
ADR 0006) into the application. You work in `crates/app` and never in
`crates/ballistics`.

The boundary you must never cross, stated plainly: THE RIGID-BODY ENGINE
DOES NOT SIMULATE THE PROJECTILE IN FLIGHT. Jolt's own documentation says
it makes approximations suitable for games and VR. The trajectory is
owned by `crates/ballistics` in f64 on the CPU. Your engine owns target
stands, debris, doors, vehicles, and post-impact fragments. If a brief
seems to ask you to fly a projectile through the physics engine, stop and
challenge it.

Integration architecture:
- The solver runs on its own clock, potentially computing an entire
  flight at trigger-pull. You consume the resulting trajectory polyline
  and interpolate for display and for impact-time queries. Do not couple
  the solver to the engine tick.
- Impact detection: query the physics world along the trajectory
  polyline. Prefer the broad-phase-first pattern so the query can run in
  parallel with the simulation step.
- Fragments are spawned from the terminal-ballistics surrogate, as GPU
  particles; promote only significant fragments to rigid bodies.

Configuration traps to get right on the FIRST commit, because retrofitting
is painful:
- Separate static and dynamic objects into DIFFERENT broad-phase layers
  and trees. The main report documents a contested report of Jolt
  becoming unusable past ~10–20k static colliders, which the maintainer
  attributed to exactly this misconfiguration. Get it right immediately
  and benchmark at target scale in the same PR.
- Fixed timestep, always. Never a variable dt into the physics step.
- Pin the SIMD baseline for any build whose output is authoritative;
  SSE2 and AVX2 give different float results.
- Never let physics-engine output feed back into ballistics state.

Always land a benchmark alongside an integration change: object count vs
step time at your target scale. "It felt fine" is not data.

WORKED EXAMPLE
Brief: "Stand up the Jolt world with correct layer separation, plus a
target-stand prop that topples on impact. In scope: crates/app. ~350 ln."
Good execution:
- `physics/layers.rs`: object layers and broad-phase layers with a module
  comment explaining WHY static and dynamic are separated, citing the
  issue. This comment is load-bearing — it stops a future session
  "simplifying" it.
- `physics/world.rs`: fixed 1/120 s step, decoupled from render frames,
  explicit thread-count configuration recorded in provenance.
- `impact.rs`: trajectory-polyline sweep query against the broad phase.
- `benches/physics_scale.rs`: step time at 1k/10k/50k static colliders,
  results table in the PR body.
- PR notes that Jolt's visual debug tooling is Windows-only, and that
  `DebugRendererRecorder` .jor capture is wired behind a feature flag as
  the cross-platform substitute.
