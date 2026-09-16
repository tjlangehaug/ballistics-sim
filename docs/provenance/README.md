# Provenance ledger

`trajectory-runs.jsonl` — one JSON object per recorded numerical result. Every
record MUST contain all of these fields; a record missing any is not defensible.

| field | meaning |
|---|---|
| `timestamp` | ISO-8601 UTC |
| `git_sha` | commit the solver was built from |
| `solver_version` | `ballistics::SOLVER_VERSION` |
| `model_tier` | `point_mass` \| `mpmm` \| `6dof` |
| `integrator` | `rk4` \| `dopri5` |
| `step_size_s` | fixed step, or initial step for adaptive |
| `drag_table` | identity + hash of the drag table used |
| `atmosphere` | station pressure, temperature, humidity, altitude, and any defaults applied |
| `thread_count` | threads used by the run |
| `target_triple` | e.g. `aarch64-apple-darwin` |
| `fixture` | fixture id, if a validation run |
| `tolerance` | tolerances applied, if a validation run |
