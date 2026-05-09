//! Wave speed primitives.
//!
//! This crate provides helpers around the wave relation
//! `speed = frequency × wavelength`, using speed in units per second,
//! frequency in hertz, and wavelength in units per cycle.
//!
//! # Examples
//!
//! ```
//! use use_wave_speed::{frequency_from_speed, wave_speed};
//!
//! let speed = wave_speed(440.0, 343.0 / 440.0).unwrap();
//! let frequency = frequency_from_speed(speed, 343.0 / 440.0).unwrap();
//!
//! assert!((speed - 343.0).abs() < 1.0e-12);
//! assert!((frequency - 440.0).abs() < 1.0e-12);
//! ```

fn is_positive_finite(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

/// Computes wave speed from frequency in hertz and wavelength.
///
/// Returns `None` when `frequency_hz` or `wavelength` is non-positive, `NaN`, or infinite,
/// or when the computed result is not finite.
///
/// # Examples
///
/// ```
/// use use_wave_speed::wave_speed;
///
/// let speed = wave_speed(440.0, 343.0 / 440.0).unwrap();
///
/// assert!((speed - 343.0).abs() < 1.0e-12);
/// ```
pub fn wave_speed(frequency_hz: f64, wavelength: f64) -> Option<f64> {
    if !is_positive_finite(frequency_hz) || !is_positive_finite(wavelength) {
        return None;
    }

    let speed = frequency_hz * wavelength;
    speed.is_finite().then_some(speed)
}

/// Computes frequency in hertz from speed and wavelength.
///
/// Returns `None` when `speed` or `wavelength` is non-positive, `NaN`, or infinite.
///
/// # Examples
///
/// ```
/// use use_wave_speed::frequency_from_speed;
///
/// let frequency = frequency_from_speed(343.0, 343.0 / 440.0).unwrap();
///
/// assert!((frequency - 440.0).abs() < 1.0e-12);
/// ```
pub fn frequency_from_speed(speed: f64, wavelength: f64) -> Option<f64> {
    if !is_positive_finite(speed) || !is_positive_finite(wavelength) {
        return None;
    }

    Some(speed / wavelength)
}

/// Computes wavelength from speed and frequency in hertz.
///
/// Returns `None` when `speed` or `frequency_hz` is non-positive, `NaN`, or infinite.
///
/// # Examples
///
/// ```
/// use use_wave_speed::wavelength_from_speed;
///
/// let wavelength = wavelength_from_speed(343.0, 440.0).unwrap();
///
/// assert!((wavelength - (343.0 / 440.0)).abs() < 1.0e-12);
/// ```
pub fn wavelength_from_speed(speed: f64, frequency_hz: f64) -> Option<f64> {
    if !is_positive_finite(speed) || !is_positive_finite(frequency_hz) {
        return None;
    }

    Some(speed / frequency_hz)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOLERANCE: f64 = 1.0e-12;

    fn assert_close(actual: f64, expected: f64) {
        assert!((actual - expected).abs() <= TOLERANCE);
    }

    #[test]
    fn computes_wave_speed() {
        assert_close(wave_speed(440.0, 343.0 / 440.0).unwrap(), 343.0);
    }

    #[test]
    fn computes_frequency_from_speed() {
        assert_close(frequency_from_speed(343.0, 343.0 / 440.0).unwrap(), 440.0);
    }

    #[test]
    fn computes_wavelength_from_speed() {
        assert_close(wavelength_from_speed(343.0, 440.0).unwrap(), 343.0 / 440.0);
    }

    #[test]
    fn preserves_wave_relationship() {
        let speed = wave_speed(50.0, 30.0).unwrap();
        assert_close(frequency_from_speed(speed, 30.0).unwrap(), 50.0);
        assert_close(wavelength_from_speed(speed, 50.0).unwrap(), 30.0);
    }

    #[test]
    fn rejects_invalid_inputs() {
        for value in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(wave_speed(value, 1.0), None);
            assert_eq!(wave_speed(1.0, value), None);
            assert_eq!(frequency_from_speed(value, 1.0), None);
            assert_eq!(frequency_from_speed(1.0, value), None);
            assert_eq!(wavelength_from_speed(value, 1.0), None);
            assert_eq!(wavelength_from_speed(1.0, value), None);
        }
    }

    #[test]
    fn rejects_non_finite_wave_speed_result() {
        assert_eq!(wave_speed(f64::MAX, 2.0), None);
    }
}
