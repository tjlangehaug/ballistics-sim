//! Unit newtypes.
//!
//! A bare `f64` carries no unit. Every quantity here is one a plain
//! `f64` could plausibly be confused with another kind of `f64` for
//! (a distance with an angle, a Mach number with an arbitrary ratio) —
//! see `CLAUDE.md` "Numerical work" and `docs/agents/ballistics.md`.

/// A distance \[m\].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Meters(f64);

impl Meters {
    /// Constructs a distance from a value in meters.
    pub fn new(meters: f64) -> Self {
        Self(meters)
    }

    /// The distance in meters \[m\].
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// A plane angle \[rad\].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Radians(f64);

impl Radians {
    /// Constructs an angle from a value in radians.
    pub fn new(radians: f64) -> Self {
        Self(radians)
    }

    /// The angle in radians \[rad\].
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// A speed \[m/s\].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MetersPerSecond(f64);

impl MetersPerSecond {
    /// Constructs a speed from a value in meters per second.
    pub fn new(meters_per_second: f64) -> Self {
        Self(meters_per_second)
    }

    /// The speed in meters per second \[m/s\].
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// The ratio of a speed to the local speed of sound. Dimensionless.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Mach(f64);

impl Mach {
    /// Constructs a Mach number from a dimensionless ratio.
    pub fn new(ratio: f64) -> Self {
        Self(ratio)
    }

    /// The Mach number as a dimensionless ratio.
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// An absolute temperature \[K\].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Kelvin(f64);

impl Kelvin {
    /// Constructs a temperature from a value in kelvin.
    ///
    /// # Panics
    /// Panics if `kelvin` is not positive — a non-positive absolute
    /// temperature is a caller bug, not a physical input to accept.
    pub fn new(kelvin: f64) -> Self {
        assert!(kelvin > 0.0, "temperature must be positive, got {kelvin} K");
        Self(kelvin)
    }

    /// The temperature in kelvin \[K\].
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// An absolute pressure \[Pa\].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Pascals(f64);

impl Pascals {
    /// Constructs a pressure from a value in pascals.
    ///
    /// # Panics
    /// Panics if `pascals` is not positive — a non-positive absolute
    /// pressure is a caller bug, not a physical input to accept.
    pub fn new(pascals: f64) -> Self {
        assert!(pascals > 0.0, "pressure must be positive, got {pascals} Pa");
        Self(pascals)
    }

    /// The pressure in pascals \[Pa\].
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// Relative humidity as a fraction in `[0.0, 1.0]` — never a percentage.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RelativeHumidity(f64);

impl RelativeHumidity {
    /// Constructs a relative humidity from a fraction in `[0.0, 1.0]`.
    ///
    /// # Panics
    /// Panics if `fraction` is outside `[0.0, 1.0]` — an out-of-range
    /// humidity is a caller bug (a percentage passed where a fraction
    /// was expected being the likely cause), not a value to clamp
    /// silently.
    pub fn new(fraction: f64) -> Self {
        assert!(
            (0.0..=1.0).contains(&fraction),
            "relative humidity must be a fraction in [0.0, 1.0], got {fraction}"
        );
        Self(fraction)
    }

    /// The relative humidity as a fraction in `[0.0, 1.0]`.
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// A mass density \[kg/m^3\].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct KilogramsPerCubicMeter(f64);

impl KilogramsPerCubicMeter {
    /// Constructs a density from a value in kilograms per cubic meter.
    ///
    /// # Panics
    /// Panics if `value` is not positive.
    pub fn new(kilograms_per_cubic_meter: f64) -> Self {
        assert!(
            kilograms_per_cubic_meter > 0.0,
            "density must be positive, got {kilograms_per_cubic_meter} kg/m^3"
        );
        Self(kilograms_per_cubic_meter)
    }

    /// The density in kilograms per cubic meter \[kg/m^3\].
    pub fn value(&self) -> f64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // These assert exact round-trips (the constructor stores the value
    // unchanged, no arithmetic happens) — not a tolerance-sensitive
    // physics comparison, so exact equality is the correct check.
    #[allow(clippy::float_cmp)]
    #[test]
    fn round_trips_the_constructed_value() {
        assert_eq!(Meters::new(1.5).value(), 1.5);
        assert_eq!(Radians::new(0.5).value(), 0.5);
        assert_eq!(MetersPerSecond::new(340.0).value(), 340.0);
        assert_eq!(Mach::new(0.9).value(), 0.9);
        assert_eq!(Kelvin::new(288.15).value(), 288.15);
        assert_eq!(Pascals::new(101_325.0).value(), 101_325.0);
        assert_eq!(RelativeHumidity::new(0.5).value(), 0.5);
        assert_eq!(KilogramsPerCubicMeter::new(1.225).value(), 1.225);
    }

    #[test]
    #[should_panic(expected = "temperature must be positive")]
    fn rejects_non_positive_temperature() {
        Kelvin::new(0.0);
    }

    #[test]
    #[should_panic(expected = "pressure must be positive")]
    fn rejects_non_positive_pressure() {
        Pascals::new(-1.0);
    }

    #[test]
    #[should_panic(expected = "relative humidity must be a fraction in [0.0, 1.0]")]
    fn rejects_out_of_range_humidity() {
        RelativeHumidity::new(1.5);
    }
}
