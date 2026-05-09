//! Wavelength primitives.
//!
//! This crate provides helpers around the wave relation
//! `speed = frequency × wavelength`, using speed in units per second,
//! frequency in hertz, and wavelength in units per cycle.
//!
//! # Examples
//!
//! ```
//! use use_wavelength::{frequency_from_wavelength, wavelength};
//!
//! let lambda = wavelength(343.0, 440.0).unwrap();
//! let frequency = frequency_from_wavelength(343.0, lambda).unwrap();
//!
//! assert!((frequency - 440.0).abs() < 1.0e-12);
//! ```

fn is_positive_finite(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

/// Computes wavelength from speed and frequency.
///
/// Returns `None` when `speed` or `frequency_hz` is non-positive, `NaN`, or infinite.
///
/// # Examples
///
/// ```
/// use use_wavelength::wavelength;
///
/// let lambda = wavelength(343.0, 440.0).unwrap();
///
/// assert!((lambda - (343.0 / 440.0)).abs() < 1.0e-12);
/// ```
pub fn wavelength(speed: f64, frequency_hz: f64) -> Option<f64> {
    if !is_positive_finite(speed) || !is_positive_finite(frequency_hz) {
        return None;
    }

    Some(speed / frequency_hz)
}

/// Computes frequency in hertz from speed and wavelength.
///
/// Returns `None` when `speed` or `wavelength` is non-positive, `NaN`, or infinite.
///
/// # Examples
///
/// ```
/// use use_wavelength::frequency_from_wavelength;
///
/// let frequency = frequency_from_wavelength(343.0, 0.5).unwrap();
///
/// assert!((frequency - 686.0).abs() < 1.0e-12);
/// ```
pub fn frequency_from_wavelength(speed: f64, wavelength: f64) -> Option<f64> {
    if !is_positive_finite(speed) || !is_positive_finite(wavelength) {
        return None;
    }

    Some(speed / wavelength)
}

/// Computes speed from frequency in hertz and wavelength.
///
/// Returns `None` when `frequency_hz` or `wavelength` is non-positive, `NaN`, or infinite,
/// or when the computed result is not finite.
///
/// # Examples
///
/// ```
/// use use_wavelength::speed_from_wavelength;
///
/// let speed = speed_from_wavelength(440.0, 343.0 / 440.0).unwrap();
///
/// assert!((speed - 343.0).abs() < 1.0e-12);
/// ```
pub fn speed_from_wavelength(frequency_hz: f64, wavelength: f64) -> Option<f64> {
    if !is_positive_finite(frequency_hz) || !is_positive_finite(wavelength) {
        return None;
    }

    let speed = frequency_hz * wavelength;
    speed.is_finite().then_some(speed)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOLERANCE: f64 = 1.0e-12;

    fn assert_close(actual: f64, expected: f64) {
        assert!((actual - expected).abs() <= TOLERANCE);
    }

    #[test]
    fn computes_wavelength_from_speed_and_frequency() {
        assert_close(wavelength(343.0, 440.0).unwrap(), 343.0 / 440.0);
    }

    #[test]
    fn computes_frequency_from_speed_and_wavelength() {
        assert_close(
            frequency_from_wavelength(343.0, 343.0 / 440.0).unwrap(),
            440.0,
        );
    }

    #[test]
    fn computes_speed_from_frequency_and_wavelength() {
        assert_close(speed_from_wavelength(440.0, 343.0 / 440.0).unwrap(), 343.0);
    }

    #[test]
    fn preserves_wave_relationship() {
        let lambda = wavelength(1500.0, 50.0).unwrap();
        assert_close(frequency_from_wavelength(1500.0, lambda).unwrap(), 50.0);
        assert_close(speed_from_wavelength(50.0, lambda).unwrap(), 1500.0);
    }

    #[test]
    fn rejects_invalid_inputs() {
        for value in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(wavelength(value, 1.0), None);
            assert_eq!(wavelength(1.0, value), None);
            assert_eq!(frequency_from_wavelength(value, 1.0), None);
            assert_eq!(frequency_from_wavelength(1.0, value), None);
            assert_eq!(speed_from_wavelength(value, 1.0), None);
            assert_eq!(speed_from_wavelength(1.0, value), None);
        }
    }

    #[test]
    fn rejects_non_finite_speed_result() {
        assert_eq!(speed_from_wavelength(f64::MAX, 2.0), None);
    }
}
