# use-wave

Composable wave primitives for Rust.

`use-wave` is the wave-specific RustUse set for lightweight helpers around
frequency, wavelength, period, and phase. It focuses on small, reusable pieces
of wave math without trying to model entire simulation or DSP systems.

## Scope

- wave relationships between speed, frequency, and wavelength
- period and angular frequency helpers
- wave number and phase calculations
- validated primitives for finite, positive wave inputs

## Workspace crates

| Crate      | Purpose                              |
| ---------- | ------------------------------------ |
| `use-wave` | Composable wave primitives for Rust. |

## Installation

Crates.io:

```toml
[dependencies]
use-wave = "0.1"
```

Local workspace or path dependency:

```toml
[dependencies]
use-wave = { path = "crates/use-wave" }
```

## Examples

Build wave properties from speed and frequency:

```rust
use use_wave::prelude::*;

let wave = WaveSpec::new(340.0, 170.0).unwrap();

assert_eq!(wave.wavelength_m(), 2.0);
assert_eq!(wave.period_s(), 1.0 / 170.0);
```

Compute phase for a time and position:

```rust
use use_wave::prelude::*;

let wave = WaveSpec::new(10.0, 2.0).unwrap();
let phase = wave.phase_radians(0.25, 1.25).unwrap();

assert!(phase.is_finite());
```

## Status

Early v0.1 API.
