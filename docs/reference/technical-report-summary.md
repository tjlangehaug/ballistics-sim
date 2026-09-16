# Technical report — condensed constraints (cite as "report §N")
Source: *Cross-Platform 3D & High-Fidelity Ballistics — Technical Research Report*, 13 Sep 2026.
This is a summary for agents. Items marked **verify** were flagged by the report itself as needing independent confirmation.

## §1 Executive summary
- The requirements describe **three programs with different clocks**: a validated 6-DOF trajectory solver, an offline continuum (fragmentation) solver, and an interactive renderer. Never run them in one loop at one time step.
- Portable GPU compute (WGSL / Metal) is **single precision only** → f64 trajectory work runs on the CPU. GPU = terminal effects, particles, rendering.
- "One machine builds all three" works for pure Rust; macOS cross-builds cannot be signed/notarised → per-OS CI is the real answer.
- Recommended stack: Rust (+ C++ via FFI only when necessary); wgpu 30; Bevy 0.18 (or bare wgpu + winit); Jolt via jolt-rust or Avian; own f64 6-DOF solver (RK4 / Dormand–Prince, tabulated aero coefficients); two-tier terminal ballistics; cargo-zigbuild locally + GitHub Actions matrix with real macOS runners.

## §2 Graphics abstraction
- wgpu 30.0.1 (MSRV 1.87), MIT/Apache-2.0; backends Vulkan/Metal/D3D12/GL; native Metal on macOS (no MoltenVK).
- Shaders: WGSL default; SPIR-V and GLSL via Naga. Compute pipelines, storage buffers, timestamp queries are core API. Ray tracing / mesh shading are **experimental** — never build a required feature on them.
- HDR via surface colour-space API; wgpu does not tone-map outside *Srgb formats.
- Costs: hidden knobs, weak GPU debugging vs Nsight, WebGPU feature ceiling, version churn.
- Vulkan + MoltenVK only if a specific extension is required. CUDA excluded from the shipped product (fine offline). OpenCL deprecated on macOS. Taichi good for authoring offline MPM.

## §3 Engine layer
- Bevy 0.18: ECS, wgpu-based, MIT/Apache-2.0; GPU-driven rendering (0.16); experimental Solari ray tracing, frame-time graph, hot-patching (0.17). Rendering will not be the bottleneck (~3.5 ms scene on RTX 4070, vendor-reported).
- **Bevy is pre-1.0 and breaks API every release** → keep the solver engine-agnostic; pin and upgrade deliberately.
- Godot 4.7: MIT, stable editor, Jolt default since 4.6, HDR output; solver becomes a GDExtension (**verify** release specifics).
- Unreal / Unity: wrong shape — engine owns the tick, determinism fights the grain, scale overwhelms a solo dev.

## §4 Rigid-body engines — for everything the projectile HITS, never the projectile
- Jolt (MIT, C++17, STL-only, no RTTI/exceptions): deterministic by design (documented limits), strong concurrency, CCD, vehicles, buoyancy. Debug viewer tooling is **Windows-only**. **Trap:** separate static and dynamic objects into different broad-phase layers from the first commit; benchmark at target scale early.
- PhysX 5.5 (BSD-3): biggest feature set, heavy. Rapier (Apache-2.0): pure Rust, **f64 variant** (`rapier3d-f64`), cross-platform deterministic. Avian: Bevy-native ECS physics, tracks Bevy releases. Box3D: **unverified**, alpha — do not plan on it.

## §5 External ballistics — the validated core
- Fidelity ladder: Tier 1 point mass + G-model (implement first — it is the validation harness); Tier 2 modified point mass (yaw of repose, spin drift, Magnus); Tier 3 full 6-DOF (needs full aero coefficient set).
- BC = form-factor scalar vs a reference projectile, assumed constant across velocity — the dominant Tier-1 error. **Custom Cd-vs-Mach tables are first-class from day one**; support G1, G2, G5, G7, GS and user tables.
- Hidden constants: a Siacci-form K = 0.355 vs naive π/8 ≈ 0.3927 (ratio 0.903 encodes Army Standard Metro). **Derive constants from stated reference atmospheres; never copy.**
- Atmosphere inputs independent: station pressure, temperature, humidity, altitude; no silent ICAO defaults. METCM/METRO optional.
- Budget for: MV temperature coefficient (50–100 fps seasonal shifts), rifle cant (2° ≈ 0.35 mil at 10 mil dial), Coriolis and Eötvös in an Earth-fixed frame.
- Implementation: f64 on CPU; RK4 baseline, adaptive Dormand–Prince upgrade (selectable, recorded); solver decoupled from engine tick (compute whole flight at trigger pull, renderer interpolates); **regression suite against published data on day one**; other open-source solvers are cross-check references, not dependencies.

## §6 Terminal ballistics
- FEM/MPM penetration + fragmentation cannot run in an interactive frame budget.
- MPM suits it: no mesh distortion, conserves mass/momentum, contact for free, fracture without remeshing, GPU-acceleratable.
- Codes (offline ground truth, none are runtime deps): OpenFDEM (strongest match), NairnMPM (material library breadth), Karamelo, MPM3D-F90 (learning), CRESSim-MPM (GPU real-time existence proof), CD-MPM, GeoTaichi (fastest readable GPU prototype).
- §6.3 two tiers: **A** offline sweep → versioned response database (penetration depth, residual velocity, exit geometry, fragment mass distribution, velocity cone); **B** runtime surrogate + GPU particles, promote significant fragments to rigid bodies, damage-state blends.
- **Gap:** validated open terminal data is largely unavailable → terminal tier is "physically principled, not empirically validated", and the product must say so.

## §7 Hardware acceleration
- Portable GPU = f32. Mitigations: keep precision work on CPU; local-origin coordinates; compensated summation only where essential; accept f32 for fragments.
- CPU parallelism is under-rated: batch trajectories (Monte Carlo, sweeps) are embarrassingly parallel with Rayon, f64, deterministic.
- GPU-shaped work: fragment/debris particles, impact visualisation textures, volumetric/field rendering. Use timestamp queries.

## §8 Toolchain and cross-compilation
- Pure Rust cross-compiles easily; any crate compiling C breaks naive cross builds.
- cargo-zigbuild: Linux (glibc pin via `.2.17` suffix) and MinGW Windows (may be buggier); glibc version check is imperfect; `+crt-static` unsupported; `-C linker` RUSTFLAGS silently bypass zig. Fallback: Docker-exported sysroot.
- macOS: osxcross needs Apple SDK (licence restricts to Apple hardware); **no signing/notarisation off-Mac**.
- Strategy: develop natively; zigbuild for smoke builds; GitHub Actions ubuntu/windows/macos matrix; macOS runner for releases; minimise C deps.

## §9 Determinism, reproducibility, tooling
- Thread count changes results in most parallel solvers → pin thread count or keep authoritative sim single-threaded.
- SIMD width changes float results → pin one instruction-set baseline for authoritative runs.
- GPU results never feed back into authoritative state.
- Record provenance with every result: solver version, integrator, step size, drag table identity, atmosphere inputs, thread count, target triple.
- Python: solver library crate first, app second; PyO3 + maturin; validation, sweeps, plotting, surrogate fitting call the same compiled solver.
- Assets: glTF 2.0, author in Blender; budget CAD→render-mesh conversion. Profiling: Bevy frame-time graph, wgpu timestamp queries, Tracy / perf / cargo-flamegraph, Jolt profiler + DebugRendererRecorder (.jor). A solver-level record/replay is worth more than any profiler.

## §10 Risk register (top items)
Critical: terminal scope swallowing the project → terminal is Phase 3, not Phase 1. High: no open validated terminal data → state it in-product. Medium-High: Bevy churn → engine-agnostic solver crate. Medium: macOS notarisation; missing 6-DOF aero coefficients (bootstrap with McDrag-class estimates, design format for measured data); Jolt broad-phase misconfiguration; solo breadth (use an engine). Low-Medium: weak GPU debugging.

## §11 Build plan
0 validation harness → 1 fidelity ladder → 2 visualisation shell (+ CI matrix) → 3 terminal Tier A → 4 terminal Tier B → 5 product surface. See `docs/plan/`.

## §12 Items to verify before committing
WGSL f64 absence against the current spec; Godot 4.6/4.7 specifics; Box3D existence. Also (Appendix A): GitHub Actions free-tier minutes/multipliers, Claude Code pricing, Apple Developer Program pricing.
