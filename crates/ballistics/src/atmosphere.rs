//! Atmospheric conditions at the firing point.
//!
//! Report §5: "Atmosphere inputs independent: station pressure,
//! temperature, humidity, altitude; no silent ICAO defaults." This
//! module holds those four inputs and the one derived quantity Phase 0
//! needs from them: air density.

use crate::units::{Kelvin, KilogramsPerCubicMeter, Meters, Pascals, RelativeHumidity};

/// Local atmospheric conditions, always specified as four independent
/// inputs — none is derived from another, and none silently falls back
/// to an ICAO standard value. A caller who wants the ICAO standard
/// sea-level reference condition must ask for it explicitly via
/// [`Atmosphere::icao_standard_sea_level`]; nothing in this crate
/// applies it automatically.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Atmosphere {
    station_pressure: Pascals,
    temperature: Kelvin,
    relative_humidity: RelativeHumidity,
    altitude: Meters,
}

impl Atmosphere {
    /// Builds an atmosphere from four independently stated inputs.
    /// There is no `Default` impl for this type: every field must be
    /// supplied by the caller, every time.
    pub fn new(
        station_pressure: Pascals,
        temperature: Kelvin,
        relative_humidity: RelativeHumidity,
        altitude: Meters,
    ) -> Self {
        Self {
            station_pressure,
            temperature,
            relative_humidity,
            altitude,
        }
    }

    /// Station pressure \[Pa\].
    pub fn station_pressure(&self) -> Pascals {
        self.station_pressure
    }

    /// Temperature \[K\].
    pub fn temperature(&self) -> Kelvin {
        self.temperature
    }

    /// Relative humidity as a fraction in `[0.0, 1.0]`.
    pub fn relative_humidity(&self) -> RelativeHumidity {
        self.relative_humidity
    }

    /// Altitude above the reference geoid \[m\].
    pub fn altitude(&self) -> Meters {
        self.altitude
    }

    /// The ICAO Standard Atmosphere's sea-level reference condition:
    /// 101 325 Pa, 288.15 K, 0% relative humidity, 0 m.
    ///
    /// This is a *named reference point*, not a default — nothing in
    /// this crate applies it automatically. It exists so
    /// [`Atmosphere::air_density`] has a published value to be checked
    /// against in a unit test.
    ///
    /// Source: ICAO Doc 7488 / ISO 2533:1975, "ICAO Standard
    /// Atmosphere" — 101 325 Pa and 288.15 K are that standard's stated
    /// sea-level reference values, quoted here as the reference
    /// condition itself, not derived from anything more fundamental.
    pub fn icao_standard_sea_level() -> Self {
        Self::new(
            Pascals::new(101_325.0),
            Kelvin::new(288.15),
            RelativeHumidity::new(0.0),
            Meters::new(0.0),
        )
    }

    /// Air density at these conditions \[kg/m^3\].
    ///
    /// Moist air is *less* dense than dry air at the same temperature
    /// and total pressure, because water's molar mass is lower than dry
    /// air's. Derivation — Dalton's law (total pressure is the sum of
    /// each gas's partial pressure) applied to the ideal gas law for
    /// each of the dry-air and water-vapor components:
    ///
    /// ```text
    /// rho = p_dry / (R_dry * T) + p_vapor / (R_vapor * T)
    /// ```
    ///
    /// where every term is derived, not pasted, from a stated source:
    /// - `R_dry = R / M_dry`. `R` = 8.314 462 618 J/(mol*K), the molar
    ///   gas constant, exact under the 2019 SI redefinition (R = N_A *
    ///   k_B, and both N_A and k_B are exactly-defined SI constants).
    ///   `M_dry` = 0.028 964 4 kg/mol, the mean molar mass of dry air
    ///   per the U.S. Standard Atmosphere, 1976 (NOAA-S/T 76-1562).
    ///   Computed here: `R_dry` ~= 287.06 J/(kg*K).
    /// - `R_vapor = R / M_water`. `M_water` = 0.018 015 28 kg/mol, the
    ///   IUPAC standard atomic-weight-derived molar mass of water
    ///   (H2O = 2 * 1.008 + 15.999 g/mol). Computed here: `R_vapor` ~=
    ///   461.5 J/(kg*K).
    /// - `p_vapor = RH * p_sat(T)`: the actual vapor partial pressure,
    ///   where `p_sat(T)` is the saturation vapor pressure at `T` via
    ///   Tetens' equation (Tetens, O., 1930, "Uber einige
    ///   meteorologische Begriffe", Zeitschrift fur Geophysik 6,
    ///   297-309; valid roughly -10 to 50 degC, comfortably covering
    ///   small-arms firing conditions):
    ///   `p_sat(T) = 610.78 * exp(17.27 * (T - 273.15) / (T - 35.85))`
    ///   \[Pa\], `T` in kelvin.
    /// - `p_dry = p_station - p_vapor`: Dalton's law, the remainder of
    ///   the station pressure after vapor's share.
    ///
    /// Check value: at [`Atmosphere::icao_standard_sea_level`] (0%
    /// humidity, so `p_vapor` = 0), this evaluates to ~1.225 kg/m^3,
    /// the Standard Atmosphere's published sea-level density — see the
    /// unit test below. That figure is a consequence of the derivation,
    /// not an input to it.
    pub fn air_density(&self) -> KilogramsPerCubicMeter {
        /// Molar gas constant \[J/(mol*K)\], exact under the 2019 SI
        /// redefinition (R = N_A * k_B).
        const MOLAR_GAS_CONSTANT: f64 = 8.314_462_618;
        /// Mean molar mass of dry air \[kg/mol\], U.S. Standard
        /// Atmosphere, 1976 (NOAA-S/T 76-1562).
        const MOLAR_MASS_DRY_AIR: f64 = 0.028_964_4;
        /// Molar mass of water \[kg/mol\] (H2O = 2*1.008 + 15.999 g/mol,
        /// IUPAC standard atomic weights).
        const MOLAR_MASS_WATER: f64 = 0.018_015_28;

        let specific_gas_constant_dry_air = MOLAR_GAS_CONSTANT / MOLAR_MASS_DRY_AIR;
        let specific_gas_constant_water_vapor = MOLAR_GAS_CONSTANT / MOLAR_MASS_WATER;

        let temperature_kelvin = self.temperature.value();
        let saturation_vapor_pressure =
            610.78 * ((17.27 * (temperature_kelvin - 273.15)) / (temperature_kelvin - 35.85)).exp();
        let vapor_pressure = self.relative_humidity.value() * saturation_vapor_pressure;
        let dry_air_pressure = self.station_pressure.value() - vapor_pressure;

        let density = dry_air_pressure / (specific_gas_constant_dry_air * temperature_kelvin)
            + vapor_pressure / (specific_gas_constant_water_vapor * temperature_kelvin);

        KilogramsPerCubicMeter::new(density)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Exact round-trip of stored fields, no arithmetic — exact equality
    // is the correct check here, not a tolerance-sensitive comparison.
    #[allow(clippy::float_cmp)]
    #[test]
    fn fields_round_trip_independently() {
        let atmosphere = Atmosphere::new(
            Pascals::new(99_000.0),
            Kelvin::new(300.0),
            RelativeHumidity::new(0.4),
            Meters::new(500.0),
        );
        assert_eq!(atmosphere.station_pressure().value(), 99_000.0);
        assert_eq!(atmosphere.temperature().value(), 300.0);
        assert_eq!(atmosphere.relative_humidity().value(), 0.4);
        assert_eq!(atmosphere.altitude().value(), 500.0);
    }

    #[test]
    fn icao_standard_sea_level_matches_the_published_density() {
        // Check value only (see air_density's doc comment) — this is
        // not a default applied anywhere, only an explicit reference
        // point used to validate the derivation above.
        let density = Atmosphere::icao_standard_sea_level().air_density().value();
        let published_sea_level_density = 1.225; // kg/m^3, ICAO Standard Atmosphere
        assert!(
            (density - published_sea_level_density).abs() < 0.001,
            "derived sea-level density {density} kg/m^3 diverges from the \
             published 1.225 kg/m^3 by more than 0.001 kg/m^3"
        );
    }

    #[test]
    fn humidity_lowers_density_at_fixed_pressure_and_temperature() {
        let dry = Atmosphere::new(
            Pascals::new(101_325.0),
            Kelvin::new(300.0),
            RelativeHumidity::new(0.0),
            Meters::new(0.0),
        );
        let humid = Atmosphere::new(
            Pascals::new(101_325.0),
            Kelvin::new(300.0),
            RelativeHumidity::new(1.0),
            Meters::new(0.0),
        );
        assert!(
            humid.air_density().value() < dry.air_density().value(),
            "saturated air at the same pressure and temperature must be \
             less dense than dry air (water's molar mass is lower)"
        );
    }
}
