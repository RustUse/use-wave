//! Phase primitives.
//!
//! This crate provides helpers for normalizing phase angles, converting between
//! degrees and radians, and computing the smallest signed phase difference.
//!
//! `normalize_radians` returns values in `[0, 2π)`.
//! `normalize_degrees` returns values in `[0, 360)`.
//! Phase difference helpers return the smallest signed wrapped difference in
//! `[-π, π)` for radians and `[-180, 180)` for degrees.
//!
//! # Examples
//!
//! ```
//! use use_phase::{normalize_degrees, phase_difference_degrees};
//!
//! let phase = normalize_degrees(725.0).unwrap();
//! let diff = phase_difference_degrees(10.0, 350.0).unwrap();
//!
//! assert!((phase - 5.0).abs() < 1.0e-12);
//! assert!((diff - 20.0).abs() < 1.0e-12);
//! ```

use std::f64::consts::PI;

fn is_finite(value: f64) -> bool {
    value.is_finite()
}

fn normalize(value: f64, cycle: f64) -> Option<f64> {
    if !is_finite(value) {
        return None;
    }

    Some(value.rem_euclid(cycle))
}

fn phase_difference(value_a: f64, value_b: f64, cycle: f64) -> Option<f64> {
    if !is_finite(value_a) || !is_finite(value_b) {
        return None;
    }

    let half_cycle = cycle / 2.0;
    let mut difference = (value_a - value_b).rem_euclid(cycle);
    if difference >= half_cycle {
        difference -= cycle;
    }

    Some(difference)
}

/// Normalizes a phase in radians into `[0, 2π)`.
///
/// Returns `None` when `phase` is `NaN` or infinite.
///
/// # Examples
///
/// ```
/// use use_phase::normalize_radians;
///
/// let normalized = normalize_radians(-std::f64::consts::PI / 2.0).unwrap();
///
/// assert!((normalized - (3.0 * std::f64::consts::PI / 2.0)).abs() < 1.0e-12);
/// ```
pub fn normalize_radians(phase: f64) -> Option<f64> {
    normalize(phase, 2.0 * PI)
}

/// Normalizes a phase in degrees into `[0, 360)`.
///
/// Returns `None` when `phase` is `NaN` or infinite.
///
/// # Examples
///
/// ```
/// use use_phase::normalize_degrees;
///
/// let normalized = normalize_degrees(725.0).unwrap();
///
/// assert!((normalized - 5.0).abs() < 1.0e-12);
/// ```
pub fn normalize_degrees(phase: f64) -> Option<f64> {
    normalize(phase, 360.0)
}

/// Computes the smallest signed wrapped phase difference in radians.
///
/// The return value is in `[-π, π)` and represents `a - b` after wrapping.
/// Returns `None` when either input is `NaN` or infinite.
///
/// # Examples
///
/// ```
/// use use_phase::phase_difference_radians;
///
/// let difference = phase_difference_radians(
///     std::f64::consts::PI / 18.0,
///     35.0 * std::f64::consts::PI / 18.0,
/// )
/// .unwrap();
///
/// assert!((difference - (std::f64::consts::PI / 9.0)).abs() < 1.0e-12);
/// ```
pub fn phase_difference_radians(a: f64, b: f64) -> Option<f64> {
    phase_difference(a, b, 2.0 * PI)
}

/// Computes the smallest signed wrapped phase difference in degrees.
///
/// The return value is in `[-180, 180)` and represents `a - b` after wrapping.
/// Returns `None` when either input is `NaN` or infinite.
///
/// # Examples
///
/// ```
/// use use_phase::phase_difference_degrees;
///
/// let difference = phase_difference_degrees(10.0, 350.0).unwrap();
///
/// assert!((difference - 20.0).abs() < 1.0e-12);
/// ```
pub fn phase_difference_degrees(a: f64, b: f64) -> Option<f64> {
    phase_difference(a, b, 360.0)
}

/// Converts degrees to radians.
///
/// Returns `None` when `degrees` is `NaN` or infinite, or when the computed result is not finite.
///
/// # Examples
///
/// ```
/// use use_phase::degrees_to_radians;
///
/// let radians = degrees_to_radians(180.0).unwrap();
///
/// assert!((radians - std::f64::consts::PI).abs() < 1.0e-12);
/// ```
pub fn degrees_to_radians(degrees: f64) -> Option<f64> {
    if !is_finite(degrees) {
        return None;
    }

    let radians = degrees * (PI / 180.0);
    radians.is_finite().then_some(radians)
}

/// Converts radians to degrees.
///
/// Returns `None` when `radians` is `NaN` or infinite, or when the computed result is not finite.
///
/// # Examples
///
/// ```
/// use use_phase::radians_to_degrees;
///
/// let degrees = radians_to_degrees(std::f64::consts::PI).unwrap();
///
/// assert!((degrees - 180.0).abs() < 1.0e-12);
/// ```
pub fn radians_to_degrees(radians: f64) -> Option<f64> {
    if !is_finite(radians) {
        return None;
    }

    let degrees = radians * (180.0 / PI);
    degrees.is_finite().then_some(degrees)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOLERANCE: f64 = 1.0e-12;

    fn assert_close(actual: f64, expected: f64) {
        assert!((actual - expected).abs() <= TOLERANCE);
    }

    #[test]
    fn normalizes_radians() {
        assert_close(normalize_radians(0.0).unwrap(), 0.0);
        assert_close(normalize_radians(2.0 * PI).unwrap(), 0.0);
        assert_close(normalize_radians(-PI / 2.0).unwrap(), 3.0 * PI / 2.0);
    }

    #[test]
    fn normalizes_degrees() {
        assert_close(normalize_degrees(0.0).unwrap(), 0.0);
        assert_close(normalize_degrees(360.0).unwrap(), 0.0);
        assert_close(normalize_degrees(725.0).unwrap(), 5.0);
        assert_close(normalize_degrees(-90.0).unwrap(), 270.0);
    }

    #[test]
    fn computes_phase_difference_in_radians() {
        assert_close(phase_difference_radians(PI / 18.0, 35.0 * PI / 18.0).unwrap(), PI / 9.0);
        assert_close(phase_difference_radians(35.0 * PI / 18.0, PI / 18.0).unwrap(), -PI / 9.0);
    }

    #[test]
    fn computes_phase_difference_in_degrees() {
        assert_close(phase_difference_degrees(10.0, 350.0).unwrap(), 20.0);
        assert_close(phase_difference_degrees(350.0, 10.0).unwrap(), -20.0);
    }

    #[test]
    fn converts_between_degrees_and_radians() {
        assert_close(degrees_to_radians(180.0).unwrap(), PI);
        assert_close(radians_to_degrees(PI).unwrap(), 180.0);
    }

    #[test]
    fn rejects_invalid_phase_inputs() {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(normalize_radians(value), None);
            assert_eq!(normalize_degrees(value), None);
            assert_eq!(degrees_to_radians(value), None);
            assert_eq!(radians_to_degrees(value), None);
            assert_eq!(phase_difference_radians(value, 0.0), None);
            assert_eq!(phase_difference_radians(0.0, value), None);
            assert_eq!(phase_difference_degrees(value, 0.0), None);
            assert_eq!(phase_difference_degrees(0.0, value), None);
        }
    }

    #[test]
    fn rejects_non_finite_angle_conversion_results() {
        assert_eq!(radians_to_degrees(f64::MAX), None);
    }
}
