//! Balance controls for stereo audio sources.
//!
//! Balance allows you to adjust the stereo panning of audio sources using
//! different panning laws (sine law, square law, or linear).

use crate::{data::object::ObsObjectTrait, sources::ObsSourceRef};

/// Type of balance/panning law to use.
///
/// Different panning laws maintain different characteristics when balancing
/// audio between left and right channels.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObsBalanceType {
    /// Sine law panning - uses sine/cosine curves for smooth power response.
    /// This maintains constant power as audio is panned from center to sides.
    /// Provides a natural-sounding pan with -3dB center attenuation.
    SineLaw = libobs::obs_balance_type_OBS_BALANCE_TYPE_SINE_LAW,

    /// Square law panning - uses square root curves.
    /// Also maintains constant power but with different characteristics than sine law.
    /// Common in professional audio mixing consoles.
    SquareLaw = libobs::obs_balance_type_OBS_BALANCE_TYPE_SQUARE_LAW,

    /// Linear panning - simple linear crossfade.
    /// Does not maintain constant power but is straightforward and predictable.
    /// Can result in a slight volume increase in the center position.
    Linear = libobs::obs_balance_type_OBS_BALANCE_TYPE_LINEAR,
}

impl From<ObsBalanceType> for u32 {
    fn from(t: ObsBalanceType) -> u32 {
        t as u32
    }
}

impl From<u32> for ObsBalanceType {
    fn from(val: u32) -> Self {
        match val {
            libobs::obs_balance_type_OBS_BALANCE_TYPE_SINE_LAW => ObsBalanceType::SineLaw,
            libobs::obs_balance_type_OBS_BALANCE_TYPE_SQUARE_LAW => ObsBalanceType::SquareLaw,
            libobs::obs_balance_type_OBS_BALANCE_TYPE_LINEAR => ObsBalanceType::Linear,
            _ => ObsBalanceType::SineLaw,
        }
    }
}

/// Extension trait for adding balance/panning methods to sources.
pub trait ObsSourceBalance {
    /// Sets the balance value for a stereo audio source.
    ///
    /// # Arguments
    /// * `balance` - Balance value where:
    ///   - -1.0 = full left
    ///   - 0.0 = center (equal on both channels)
    ///   - 1.0 = full right
    ///
    /// Values outside this range may be clamped depending on the implementation.
    ///
    /// # Example
    /// ```no_run
    /// use libobs_wrapper::audio::ObsSourceBalance;
    /// # use libobs_wrapper::sources::ObsSourceRef;
    /// # fn example(source: &ObsSourceRef) {
    /// // Pan the source 50% to the right
    /// source.set_balance_value(0.5);
    ///
    /// // Pan fully to the left
    /// source.set_balance_value(-1.0);
    ///
    /// // Center (equal on both channels)
    /// source.set_balance_value(0.0);
    /// # }
    /// ```
    fn set_balance_value(&self, balance: f32);

    /// Gets the current balance value for a stereo audio source.
    ///
    /// # Returns
    /// Balance value in the range [-1.0, 1.0] where:
    /// - -1.0 = full left
    /// - 0.0 = center
    /// - 1.0 = full right
    fn get_balance_value(&self) -> f32;
}

impl ObsSourceBalance for ObsSourceRef {
    fn set_balance_value(&self, balance: f32) {
        unsafe {
            libobs::obs_source_set_balance_value(self.as_ptr().get_ptr(), balance);
        }
    }

    fn get_balance_value(&self) -> f32 {
        unsafe { libobs::obs_source_get_balance_value(self.as_ptr().get_ptr()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balance_type_conversion() {
        assert_eq!(
            ObsBalanceType::SineLaw as u32,
            libobs::obs_balance_type_OBS_BALANCE_TYPE_SINE_LAW
        );
        assert_eq!(
            ObsBalanceType::SquareLaw as u32,
            libobs::obs_balance_type_OBS_BALANCE_TYPE_SQUARE_LAW
        );
        assert_eq!(
            ObsBalanceType::Linear as u32,
            libobs::obs_balance_type_OBS_BALANCE_TYPE_LINEAR
        );
    }
}
