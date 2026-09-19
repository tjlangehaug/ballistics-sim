# 0007 — Published-fixture file format
Status: Accepted
Date: 2026-09-19

## Context
Phase 0's exit criterion is one published reference trajectory reproduced
within tolerance in CI (report §11; `docs/plan/phase-0-validation.md`).
Every fixture needs: projectile + atmosphere inputs, a cited published
source, a per-range table of expected drop/drift/velocity/TOF, and a
tolerance per column with a physical rationale (`docs/agents/test.md`).
The format must be readable next to the published table it transcribes
(a reviewer needs to eyeball-check it), must carry inline citation
comments, and must be parsed by both `crates/ballistics/tests/` (Rust)
and, later, Python tooling (`docs/agents/pybind.md`).

## Options
1. **TOML, one file per fixture, metadata + `[[range]]` array-of-tables**
   — human-readable, supports `#` comments so the citation and any
   per-row anomaly note sit next to the data they describe, one parser
   (`serde` + `toml`) for the whole fixture, and it matches the format
   already used elsewhere in this repo (`Cargo.toml`,
   `rust-toolchain.toml`). Cost: array-of-tables is more verbose per row
   than CSV; a large table (hundreds of rows) would be unpleasant to
   read or diff.
2. **CSV data + sidecar YAML/TOML metadata file (two files per fixture)**
   — CSV is the natural shape for a published range/drop/drift/velocity
   table and is trivial to diff row-by-row against a source PDF. Cost:
   two files per fixture to keep in sync, no native per-row comments in
   CSV (an anomaly note needs an extra column or a metadata cross-ref),
   two parsers.
3. **RON** — native Rust literal syntax, no serde-mapping friction,
   supports comments. Cost: unfamiliar to anyone checking the file
   against a published paper table (RON's syntax is Rust-specific
   tooling, not something a reviewer without Rust context can scan);
   no advantage over TOML here since both need serde either way.

## Decision
TOML, one file per fixture. Published external-ballistics reference
tables are small (a handful to a few dozen range points per source) —
CSV's density advantage doesn't matter at that size, and the one-file,
one-parser, comment-friendly citation trail matters at every size.

## Consequences
- Easier: one fixture format for both the Rust harness and future
  Python tooling to parse; citations and anomaly notes live next to the
  data; consistent with repo convention.
- Harder: nothing significant at expected fixture sizes. If a future
  fixture source is a dense table (hundreds of rows — e.g. a fine-grained
  Doppler-radar trace), TOML's array-of-tables will be noticeably more
  verbose than CSV.
- `serde` + `toml` become new dependencies of the crate(s) that parse
  fixtures. Per `CLAUDE.md`, whichever PR first adds them must carry its
  own ADR — this document decides the *format*, not the dependency.

## Revisit if
A published source arrives as a dense table (hundreds of rows) where
TOML's verbosity makes the fixture materially harder to review than a
CSV would.
