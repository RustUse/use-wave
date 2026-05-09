//! Frequency primitives and conversions.
//!
//! This crate provides small helpers for converting between frequency in hertz,
//! period in seconds, and angular frequency in radians per second.
//!
//! # Examples
//!
//! ```
//! use use_frequency::{frequency_to_angular_frequency, frequency_to_period};
//!
//! let period = frequency_to_period(2.0).unwrap();
//! let angular = frequency_to_angular_frequency(2.0).unwrap();
//!
//! assert!((period - 0.5).abs() < 1.0e-12);
//! assert!((angular - 4.0 * std::f64::consts::PI).abs() < 1.0e-12);
//! ```

use std::f64::consts::PI;

fn is_positive_finite(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

/// Converts frequency in hertz to period in seconds.
///
/// Returns `None` when `frequency_hz` is zero, negative, `NaN`, or infinite.
///
/// # Examples
///
/// ```
/// use use_frequency::frequency_to_period;
///
/// let period = frequency_to_period(4.0).unwrap();
///
/// assert!((period - 0.25).abs() < 1.0e-12);
/// ```
pub fn frequency_to_period(frequency_hz: f64) -> Option<f64> {
    if !is_positive_finite(frequency_hz) {
        return None;
    }

    Some(1.0 / frequency_hz)
}

/// Converts period in seconds to frequency in hertz.
///
/// Returns `None` when `period_seconds` is zero, negative, `NaN`, or infinite.
///
/// # Examples
///
/// ```
/// use use_frequency::period_to_frequency;
///
/// let frequency = period_to_frequency(0.5).unwrap();
///
/// assert!((frequency - 2.0).abs() < 1.0e-12);
/// ```
pub fn period_to_frequency(period_seconds: f64) -> Option<f64> {
    if !is_positive_finite(period_seconds) {
        return None;
    }

    Some(1.0 / period_seconds)
}

/// Converts frequency in hertz to angular frequency in radians per second.
///
/// Returns `None` when `frequency_hz` is zero, negative, `NaN`, or infinite,
/// or when the computed result is not finite.
///
/// # Examples
///
/// ```
/// use use_frequency::frequency_to_angular_frequency;
///
/// let angular = frequency_to_angular_frequency(1.0).unwrap();
///
/// assert!((angular - 2.0 * std::f64::consts::PI).abs() < 1.0e-12);
/// ```
pub fn frequency_to_angular_frequency(frequency_hz: f64) -> Option<f64> {
    if !is_positive_finite(frequency_hz) {
        return None;
    }

    let angular_frequency = 2.0 * PI * frequency_hz;
    angular_frequency.is_finite().then_some(angular_frequency)
}

/// Converts angular frequency in radians per second to frequency in hertz.
///
/// Returns `None` when `angular_frequency` is zero, negative, `NaN`, or infinite.
///
/// # Examples
///
/// ```
/// use use_frequency::angular_frequency_to_frequency;
///
/// let frequency = angular_frequency_to_frequency(4.0 * std::f64::consts::PI).unwrap();
///
/// assert!((frequency - 2.0).abs() < 1.0e-12);
/// ```
pub fn angular_frequency_to_frequency(angular_frequency: f64) -> Option<f64> {
    if !is_positive_finite(angular_frequency) {
        return None;
    }

    Some(angular_frequency / (2.0 * PI))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOLERANCE: f64 = 1.0e-12;

    fn assert_close(actual: f64, expected: f64) {
        assert!((actual - expected).abs() <= TOLERANCE);
    }

    #[test]
    fn converts_frequency_to_period() {
        assert_close(frequency_to_period(440.0).unwrap(), 1.0 / 440.0);
    }

    #[test]
    fn converts_period_to_frequency() {
        assert_close(period_to_frequency(0.25).unwrap(), 4.0);
    }

    #[test]
    fn converts_frequency_to_angular_frequency() {
        assert_close(
            frequency_to_angular_frequency(440.0).unwrap(),
            2.0 * PI * 440.0,
        );
    }

    #[test]
    fn converts_angular_frequency_to_frequency() {
        assert_close(angular_frequency_to_frequency(8.0 * PI).unwrap(), 4.0);
    }

    #[test]
    fn rejects_invalid_frequency_inputs() {
        for value in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(frequency_to_period(value), None);
            assert_eq!(frequency_to_angular_frequency(value), None);
        }
    }

    #[test]
    fn rejects_invalid_period_inputs() {
        for value in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(period_to_frequency(value), None);
        }
    }

    #[test]
    fn rejects_invalid_angular_frequency_inputs() {
        for value in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(angular_frequency_to_frequency(value), None);
        }
    }

    #[test]
    fn rejects_non_finite_angular_frequency_results() {
        assert_eq!(frequency_to_angular_frequency(f64::MAX), None);
    }
}
