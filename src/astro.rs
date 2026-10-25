//! Astronomical arguments used by harmonic tidal analysis.
//!
//! Time is represented as [`Days`], the number of days since
//! `1970-01-01 00:00:00 UTC`. This module currently uses UTC as an
//! approximation of UT1 and does not apply ΔT. Those differences are small
//! for the initial research prototype, but should be made configurable when
//! high-precision historical analysis is added.
//!
//! Mean longitudes are calculated with the polynomials from Jean Meeus,
//! *Astronomical Algorithms*, 2nd edition, chapters 7, 25, 47 and 50.
//!
//! The internal time conversion uses Julian dates:
//! `JD = days_since_unix_epoch + 2_440_587.5`.
//!

/// Number of UTC days since the Unix epoch.
pub type Days = f64;

/// Julian date of `1970-01-01 00:00:00 UTC`.
pub const UNIX_EPOCH_JD: f64 = 2_440_587.5;
/// Julian date of the J2000.0 epoch (`2000-01-01 12:00:00`).
pub const J2000_JD: f64 = 2_451_545.0;

/// Number of hours in a Julian century.
pub const HOURS_PER_JULIAN_CENTURY: f64 = 36_525.0 * 24.0;

/// Mean lunar longitude rate, in degrees per hour.
pub const RATE_S: f64 = 481_267.881_234_21 / HOURS_PER_JULIAN_CENTURY;
/// Mean solar longitude rate, in degrees per hour.
pub const RATE_H: f64 = 36_000.769_83 / HOURS_PER_JULIAN_CENTURY;
/// Mean lunar perigee longitude rate, in degrees per hour.
pub const RATE_P: f64 = 4_069.013_728_7 / HOURS_PER_JULIAN_CENTURY;
/// Rate of the negative lunar node longitude (`N' = -N`), in degrees per hour.
pub const RATE_NP: f64 = 1_934.136_289_1 / HOURS_PER_JULIAN_CENTURY;
/// Mean terrestrial perihelion longitude rate, in degrees per hour.
pub const RATE_P1: f64 = 1.719_46 / HOURS_PER_JULIAN_CENTURY;
/// Mean lunar hour-angle rate, in degrees per hour.
pub const RATE_TAU: f64 = 15.0 + RATE_H - RATE_S;

/// Rates of the six Doodson variables in `[tau, s, h, p, N', p1]` order.
pub const DOODSON_RATES: [f64; 6] = [RATE_TAU, RATE_S, RATE_H, RATE_P, RATE_NP, RATE_P1];

/// Normalizes an angle to the half-open interval `[0, 360)` degrees.
#[inline]
pub fn wrap360(x: f64) -> f64 {
    let remainder = x % 360.0;
    if remainder < 0.0 {
        remainder + 360.0
    } else {
        remainder
    }
}

/// Returns Julian centuries elapsed since J2000.0.
#[inline]
pub fn julian_centuries(t: Days) -> f64 {
    (t + UNIX_EPOCH_JD - J2000_JD) / 36_525.0
}

/// Astronomical arguments at one instant.
#[derive(Debug, Clone, Copy)]
pub struct AstroArgs {
    /// Doodson arguments `[tau, s, h, p, N', p1]` in degrees.
    pub doodson: [f64; 6],
    /// Mean longitude of the ascending lunar node `N`, in degrees.
    pub node_deg: f64,
}

impl AstroArgs {
    /// Returns the ascending-node longitude in radians.
    #[inline]
    pub fn node_rad(&self) -> f64 {
        self.node_deg.to_radians()
    }
}

/// Calculates astronomical arguments for `t` days since the Unix epoch.
///
/// The returned angles are normalized to `[0, 360)` degrees. The `tau`
/// argument uses the fractional UTC day and is therefore suitable for the
/// initial UTC-based model, not for a precision UT1 implementation.
pub fn astro_args(t: Days) -> AstroArgs {
    let tc = julian_centuries(t);
    let tc2 = tc * tc;
    let tc3 = tc2 * tc;
    let tc4 = tc3 * tc;

    // Mean lunar longitude (s).
    let s: f64 = 218.316_447_7 + 481_267.881_234_21 * tc - 0.001_578_6 * tc2 + tc3 / 538_841.0
        - tc4 / 65_194_000.0;
    // Mean solar longitude (h).
    let h: f64 = 280.466_46 + 36_000.769_83 * tc + 0.000_303_2 * tc2;
    // Mean longitude of the lunar perigee (p).
    let p: f64 = 83.353_246_5 + 4_069.013_728_7 * tc + 0.010_320_0 * tc2 - tc3 / 80_053.0
        + tc4 / 18_999_000.0;
    // Mean longitude of the ascending lunar node (N).
    let n: f64 = 125.044_547_9 - 1_934.136_289_1 * tc + 0.002_075_4 * tc2 + tc3 / 467_441.0
        - tc4 / 60_616_000.0;
    // Mean longitude of the Earth's perihelion (p1).
    let p1: f64 = 282.937_35 + 1.719_46 * tc + 0.000_46 * tc2;

    let s: f64 = wrap360(s);
    let h: f64 = wrap360(h);
    let p: f64 = wrap360(p);
    let n: f64 = wrap360(n);
    let p1: f64 = wrap360(p1);

    // UTC hour of day and mean Sun hour angle.
    let hours = t.rem_euclid(1.0) * 24.0;
    let tau = wrap360(15.0 * hours + 180.0 + h - s);

    AstroArgs {
        doodson: [tau, s, h, p, wrap360(-n), p1],
        node_deg: n,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_positive_and_negative_angles() {
        assert_eq!(wrap360(360.0), 0.0);
        assert_eq!(wrap360(-30.0), 330.0);
        assert_eq!(wrap360(725.0), 5.0);
    }

    #[test]
    fn unix_epoch_is_before_j2000_by_three_tenths_of_a_century() {
        assert!((julian_centuries(0.0) + 0.3).abs() < 1e-15);
    }

    #[test]
    fn astronomical_arguments_are_normalized() {
        let args = astro_args(0.0);

        assert!(
            args.doodson
                .iter()
                .all(|angle| (0.0..360.0).contains(angle))
        );
        assert!((args.node_rad() - args.node_deg.to_radians()).abs() < f64::EPSILON);
    }
}
