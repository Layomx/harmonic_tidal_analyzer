//! Astronomics time and arguments
//!
//! Time is represented as **days since 1970-01-01T00:00:00 UTC** (`f64`).
//! The difference UT1-UTC (< 1 s) and ΔT (~69 s) are ignored: their effect on the phases of the arguments in on the order of 1e-3 degrees in the semidiurnal bands, negligible compared of the statistical uncertainty
//!
//! Mean longitudes are calculated using the polynomials from Meeus, Astronomical Algorithms. Second Edition. Caps 7, 25, 47 and 50.
//!
//! Julian dates are calculated with: `JD = ⌊365.25(Y + 4716)⌋ + ⌊30.6001(M + 1)⌋ + D + B - 1524.5`
//!
//!

/// Days (UTC)
pub type Days = f64;

/// Julian dates
pub const UNIX_EPOCH_JD: f64 = 2_440_587.5; // JD of 1970-01-01T00:00:00 UTC
pub const J2000_JD: f64 = 2_451_545.0; // JD of 2000-01-01T12:00:00 TT

// Hours per Julian century
pub const HOURS_PER_JULIAN_CENTURY: f64 = 36_525.0 * 24.0;
// Average rates of the astronomical variables in degrees per hour
pub const RATE_S: f64 = 481_267.881_234_21 / HOURS_PER_JULIAN_CENTURY;
pub const RATE_H: f64 = 36_000.769_83 / HOURS_PER_JULIAN_CENTURY;
pub const RATE_P: f64 = 4_069.013_728_7 / HOURS_PER_JULIAN_CENTURY;
/// N' = -N (lunar node returns)
pub const RATE_NP: f64 = 1_934.136_289_1 / HOURS_PER_JULIAN_CENTURY;
pub const RATE_P1: f64 = 1.719_46 / HOURS_PER_JULIAN_CENTURY;
pub const RATE_TAU: f64 = 15.0 / RATE_H - RATE_S;
