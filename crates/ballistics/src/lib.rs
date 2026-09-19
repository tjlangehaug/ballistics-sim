//! External ballistics solver — the core IP of this project.
//!
//! Invariants (see `CLAUDE.md` and `docs/decisions/0003-f64-cpu-authoritative-solver.md`):
//! - f64 throughout; CPU only; no graphics dependencies.
//! - Deterministic: same inputs and thread count give bit-identical output.
//! - Every public item documents its units.

pub mod units;

/// Solver version recorded in every provenance record
/// (`docs/provenance/README.md`). Dimensionless string, semver.
pub const SOLVER_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::SOLVER_VERSION;

    #[test]
    fn solver_version_is_embedded() {
        assert!(!SOLVER_VERSION.is_empty());
    }
}
