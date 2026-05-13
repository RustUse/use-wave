# use-wave

Composable wave primitives for Rust.

`use-wave` provides validated helpers for wavelength, period, angular
frequency, wave number, and phase.

## Examples

```rust
use use_wave::prelude::*;

let wave = WaveSpec::new(340.0, 170.0).unwrap();

assert_eq!(wave.wavelength_m(), 2.0);
assert_eq!(wave.period_s(), 1.0 / 170.0);
```
