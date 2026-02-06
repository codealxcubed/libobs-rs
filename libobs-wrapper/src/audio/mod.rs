//! Audio control and monitoring features for libobs.
//!
//! This module provides Rust wrappers for libobs audio control features including:
//! - **Faders**: Control audio levels with different mapping types (Cubic, IEC, Logarithmic)
//! - **Volume Meters**: Monitor peak and RMS audio levels
//! - **Audio Monitoring**: Configure per-source audio monitoring
//! - **Balance Control**: Adjust stereo balance with different panning laws
//! - **Utility Functions**: Convert between dB, multiplier, and deflection values
//!
//! ## Multi-Channel Audio Support
//!
//! Multi-channel audio (5.1, 7.1 surround sound) is supported through the audio system.
//! Configure speaker layouts using [`crate::enums::ObsSpeakerLayout`] when setting up the
//! audio context with [`crate::data::audio::ObsAudioInfo`].
//!
//! ## Audio Ducking (Platform-Specific)
//!
//! Windows audio ducking is a platform-specific feature implemented in OBS Studio's frontend
//! using Windows COM APIs (IAudioSessionControl2::SetDuckingPreference). This functionality
//! is not exposed through libobs itself.
//!
//! If you need audio ducking in your Rust application on Windows, you'll need to use
//! the Windows crate to call the COM APIs directly. For reference, see OBS Studio's
//! implementation in `frontend/utility/platform-windows.cpp`.

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
