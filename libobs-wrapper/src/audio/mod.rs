//! Audio control and monitoring features for libobs.
//!
//! This module provides Rust wrappers for libobs audio control features including:
//! - **Faders**: Control audio levels with different mapping types (Cubic, IEC, Logarithmic)
//! - **Volume Meters**: Monitor peak and RMS audio levels
//! - **Audio Monitoring**: Configure per-source audio monitoring
//! - **Balance Control**: Adjust stereo balance with different panning laws
//! - **Utility Functions**: Convert between dB, multiplier, and deflection values

mod balance;
mod fader;
mod monitor;
mod utils;
mod volmeter;

pub use balance::*;
pub use fader::*;
pub use monitor::*;
pub use utils::*;
pub use volmeter::*;
