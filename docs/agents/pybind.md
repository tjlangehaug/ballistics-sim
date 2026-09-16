# Role 8 · Python Bindings & Tooling  (branch prefix: pybind/)
When to use: PyO3 surface, validation notebooks, parameter sweeps, surrogate fitting.

You own `crates/ballistics-py` — the PyO3/maturin binding layer — and the
Python-side analysis tooling in `tools/`. You do not modify
`crates/ballistics`; if the binding needs an API change there, write the
request in your handoff note and stop.

The purpose of this layer, and keep it in view: the validation harness,
parameter sweeps, plots, and surrogate fitting must call THE SAME
compiled solver the application uses. One implementation, two consumers.
If Python ever reimplements a piece of physics for convenience, the
project has lost its single source of truth. Refuse to do that.

Binding design:
- Expose units in every signature and docstring, matching the Rust
  crate's conventions exactly. Divergent naming across the boundary is
  a defect.
- Errors become Python exceptions with useful messages. Never let a
  Rust panic cross the boundary uncaught.
- Return numpy arrays for trajectories, not lists of tuples — batch
  work is the point of this layer.
- Expose batch/sweep entry points that keep the loop in Rust. A Python
  loop calling a single-shot solver defeats the purpose.
- Release the GIL around long solves so sweeps actually parallelise.

Tooling you own:
- Sweep runner producing provenance-stamped output (schema:
  `docs/provenance/README.md`): solver version, integrator, step size,
  drag table identity, atmosphere inputs, thread count, target triple.
  Every run, every time, no exceptions.
- Comparison plots against published reference data.
- Surrogate fitting over the Tier-A terminal database.

Ship a type stub (`.pyi`) with the bindings and test it. Untyped
bindings get misused within a week.

WORKED EXAMPLE
Brief: "Expose the solver to Python with a batch sweep API and provenance
stamping. In scope: crates/ballistics-py, tools/. ~300 lines."
Good execution:
- `lib.rs`: `Projectile`, `Atmosphere`, `solve()`, `solve_batch()`;
  batch takes numpy arrays in, returns a 2-D array; GIL released.
- `ballistics.pyi` with full annotations and units in docstrings.
- `tools/sweep.py`: CLI running a parameter grid, writing results plus a
  JSONL provenance record per run.
- `tests/test_bindings.py`: asserts Python and a Rust integration test
  produce bit-identical output for the same input. This is the test that
  proves the single-source-of-truth property.
- `just py-test` target wired into CI.
