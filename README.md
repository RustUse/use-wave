# use-wave

Composable wave primitives for Rust.

`use-wave` provides small utilities for frequency, period, wavelength, wave speed, angular frequency, and phase.

Future sibling sets can build on top of these primitives:
- `use-acoustics` can build sound-specific helpers on top of this.
- `use-optics` can build light-specific helpers on top of this.
- `use-signal` can build sampled-signal helpers on top of this.
- `use-physics` can build broader physical relationships on top of this.

## Workspace crates

- `use-frequency`: frequency and angular frequency conversions.
- `use-period`: period conversions and angular frequency from period.
- `use-wavelength`: wavelength, frequency, and speed relationships.
- `use-wave-speed`: wave speed, frequency, and wavelength relationships.
- `use-phase`: phase normalization, phase differences, and angle conversions.

## Examples

```rust
use use_frequency::{frequency_to_angular_frequency, frequency_to_period};
use use_phase::normalize_degrees;
use use_wave_speed::wave_speed;
use use_wavelength::wavelength;

let frequency_hz = 440.0;

let period = frequency_to_period(frequency_hz).unwrap();
let angular = frequency_to_angular_frequency(frequency_hz).unwrap();

let speed_of_sound = 343.0;
let lambda = wavelength(speed_of_sound, frequency_hz).unwrap();

let speed = wave_speed(frequency_hz, lambda).unwrap();

let phase = normalize_degrees(725.0).unwrap();

assert!((period - 1.0 / 440.0).abs() < 1.0e-12);
assert!((angular - 2.0 * std::f64::consts::PI * 440.0).abs() < 1.0e-12);
assert!((speed - 343.0).abs() < 1.0e-12);
assert!((phase - 5.0).abs() < 1.0e-12);
```
