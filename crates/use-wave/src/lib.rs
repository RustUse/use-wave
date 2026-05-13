#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use std::{error::Error, f64::consts::TAU, fmt};

pub mod prelude;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WaveSpec {
    speed_m_per_s: f64,
    frequency_hz: f64,
}

impl WaveSpec {
    pub fn new(speed_m_per_s: f64, frequency_hz: f64) -> Result<Self, WaveError> {
        validate_positive_finite(speed_m_per_s)?;
        validate_positive_finite(frequency_hz)?;

        Ok(Self {
            speed_m_per_s,
            frequency_hz,
        })
    }

    #[must_use]
    pub const fn speed_m_per_s(self) -> f64 {
        self.speed_m_per_s
    }

    #[must_use]
    pub const fn frequency_hz(self) -> f64 {
        self.frequency_hz
    }

    #[must_use]
    pub fn wavelength_m(self) -> f64 {
        self.speed_m_per_s / self.frequency_hz
    }

    #[must_use]
    pub fn period_s(self) -> f64 {
        1.0 / self.frequency_hz
    }

    #[must_use]
    pub fn angular_frequency_rad_s(self) -> f64 {
        TAU * self.frequency_hz
    }

    #[must_use]
    pub fn wave_number_rad_m(self) -> f64 {
        TAU / self.wavelength_m()
    }

    pub fn phase_radians(self, time_s: f64, position_m: f64) -> Result<f64, WaveError> {
        validate_finite(time_s)?;
        validate_finite(position_m)?;

        Ok(self.angular_frequency_rad_s() * time_s - self.wave_number_rad_m() * position_m)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WaveError {
    NonFinite,
    NonPositive,
}

impl fmt::Display for WaveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite => formatter.write_str("wave values must be finite"),
            Self::NonPositive => formatter.write_str("wave values must be greater than zero"),
        }
    }
}

impl Error for WaveError {}

pub fn wavelength_meters(speed_m_per_s: f64, frequency_hz: f64) -> Result<f64, WaveError> {
    Ok(WaveSpec::new(speed_m_per_s, frequency_hz)?.wavelength_m())
}

pub fn frequency_hz(speed_m_per_s: f64, wavelength_m: f64) -> Result<f64, WaveError> {
    validate_positive_finite(speed_m_per_s)?;
    validate_positive_finite(wavelength_m)?;

    Ok(speed_m_per_s / wavelength_m)
}

pub fn period_seconds(frequency_hz: f64) -> Result<f64, WaveError> {
    validate_positive_finite(frequency_hz)?;

    Ok(1.0 / frequency_hz)
}

fn validate_positive_finite(value: f64) -> Result<(), WaveError> {
    validate_finite(value)?;

    if value <= 0.0 {
        return Err(WaveError::NonPositive);
    }

    Ok(())
}

fn validate_finite(value: f64) -> Result<(), WaveError> {
    if !value.is_finite() {
        return Err(WaveError::NonFinite);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{WaveError, WaveSpec, frequency_hz, period_seconds, wavelength_meters};

    #[test]
    fn computes_wave_relationships() {
        let wave = WaveSpec::new(340.0, 170.0).expect("wave should be valid");

        assert_eq!(wave.wavelength_m(), 2.0);
        assert_eq!(wave.period_s(), 1.0 / 170.0);
        assert_eq!(wavelength_meters(340.0, 170.0), Ok(2.0));
        assert_eq!(frequency_hz(340.0, 2.0), Ok(170.0));
        assert_eq!(period_seconds(4.0), Ok(0.25));
    }

    #[test]
    fn computes_phase_and_wave_numbers() {
        let wave = WaveSpec::new(10.0, 2.0).expect("wave should be valid");
        let phase = wave
            .phase_radians(0.25, 1.25)
            .expect("phase inputs should be valid");

        assert!(phase.is_finite());
        assert!(wave.angular_frequency_rad_s() > 0.0);
        assert!(wave.wave_number_rad_m() > 0.0);
    }

    #[test]
    fn rejects_invalid_inputs() {
        assert_eq!(WaveSpec::new(0.0, 1.0), Err(WaveError::NonPositive));
        assert_eq!(WaveSpec::new(1.0, f64::NAN), Err(WaveError::NonFinite));
        assert_eq!(frequency_hz(1.0, -1.0), Err(WaveError::NonPositive));
        assert_eq!(period_seconds(f64::INFINITY), Err(WaveError::NonFinite));
    }
}
