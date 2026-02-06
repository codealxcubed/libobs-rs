//! Fader controls for managing audio levels.
//!
//! A fader is used to map input values from UI controls to dB and multiplier values
//! that libobs uses to mix audio. The fader internally stores its position as a dB value.

use crate::{data::object::ObsObjectTrait, sources::ObsSourceRef, utils::ObsError};

/// Type of fader curve to use for level mapping.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObsFaderType {
    /// A simple cubic fader for controlling audio levels.
    /// Maps input values using the formula x³, which yields good results
    /// while being performant. This is a very common software fader type.
    Cubic = libobs::obs_fader_type_OBS_FADER_CUBIC,

    /// A fader compliant to IEC 60-268-18.
    /// Uses multiple segments with different slopes that map deflection
    /// linearly to dB values, providing accurate professional-grade control.
    IEC = libobs::obs_fader_type_OBS_FADER_IEC,

    /// Logarithmic fader providing smooth perceptual volume control.
    Log = libobs::obs_fader_type_OBS_FADER_LOG,
}

impl From<ObsFaderType> for u32 {
    fn from(t: ObsFaderType) -> u32 {
        t as u32
    }
}

/// A fader for controlling audio source levels.
///
/// The fader maps UI input values to dB values and multiplier values that libobs
/// uses for audio mixing. It can be attached to a source to automatically sync
/// with the source's volume.
///
/// # Example
/// ```no_run
/// use libobs_wrapper::audio::{ObsFader, ObsFaderType};
///
/// // Create a cubic fader
/// let fader = ObsFader::new(ObsFaderType::Cubic)?;
///
/// // Set the level to -6 dB
/// fader.set_db(-6.0);
///
/// // Get the multiplier value for mixing
/// let mul = fader.get_mul();
///
/// // Attach to a source (requires initialized OBS context and source)
/// // fader.attach_source(&source);
/// # Ok::<(), libobs_wrapper::utils::ObsError>(())
/// ```
pub struct ObsFader {
    inner: *mut libobs::obs_fader_t,
}

impl ObsFader {
    /// Creates a new fader with the specified type.
    ///
    /// # Arguments
    /// * `fader_type` - The type of fader curve to use
    ///
    /// # Returns
    /// A new `ObsFader` instance, or an error if creation failed
    pub fn new(fader_type: ObsFaderType) -> Result<Self, ObsError> {
        let inner = unsafe { libobs::obs_fader_create(fader_type as u32) };

        if inner.is_null() {
            return Err(ObsError::NullPointer(Some(
                "Failed to create fader".to_string(),
            )));
        }

        Ok(Self { inner })
    }

    /// Sets the fader dB value.
    ///
    /// # Arguments
    /// * `db` - The new dB value to set
    ///
    /// # Returns
    /// `true` if the value was set without clamping, `false` if it was clamped to limits
    pub fn set_db(&self, db: f32) -> bool {
        unsafe { libobs::obs_fader_set_db(self.inner, db) }
    }

    /// Gets the current fader dB value.
    pub fn get_db(&self) -> f32 {
        unsafe { libobs::obs_fader_get_db(self.inner) }
    }

    /// Sets the fader value from a deflection value.
    ///
    /// Deflection is typically in the range [0.0, 1.0] but may be higher for amplification.
    ///
    /// # Arguments
    /// * `def` - The deflection value to set
    ///
    /// # Returns
    /// `true` if the value was set without clamping, `false` if it was clamped to limits
    pub fn set_deflection(&self, def: f32) -> bool {
        unsafe { libobs::obs_fader_set_deflection(self.inner, def) }
    }

    /// Gets the current fader deflection value.
    pub fn get_deflection(&self) -> f32 {
        unsafe { libobs::obs_fader_get_deflection(self.inner) }
    }

    /// Sets the fader value from a multiplier.
    ///
    /// # Arguments
    /// * `mul` - The multiplier value to set
    ///
    /// # Returns
    /// `true` if the value was set without clamping, `false` if it was clamped to limits
    pub fn set_mul(&self, mul: f32) -> bool {
        unsafe { libobs::obs_fader_set_mul(self.inner, mul) }
    }

    /// Gets the current fader multiplier value.
    ///
    /// This is the actual multiplier that will be applied to audio samples.
    pub fn get_mul(&self) -> f32 {
        unsafe { libobs::obs_fader_get_mul(self.inner) }
    }

    /// Attaches the fader to a source.
    ///
    /// When attached, the fader automatically syncs its state to the source's volume.
    ///
    /// # Arguments
    /// * `source` - The source to attach to
    ///
    /// # Returns
    /// `true` if attachment succeeded, `false` otherwise
    pub fn attach_source(&self, source: &ObsSourceRef) -> bool {
        unsafe { libobs::obs_fader_attach_source(self.inner, source.as_ptr().get_ptr()) }
    }

    /// Detaches the fader from its currently attached source.
    pub fn detach_source(&self) {
        unsafe { libobs::obs_fader_detach_source(self.inner) }
    }

    /// Returns the raw pointer to the fader.
    ///
    /// # Safety
    /// The caller must ensure the pointer is used safely and doesn't outlive the fader.
    pub fn as_ptr(&self) -> *mut libobs::obs_fader_t {
        self.inner
    }
}

impl Drop for ObsFader {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                libobs::obs_fader_destroy(self.inner);
            }
        }
    }
}

// Faders are thread-safe according to libobs design
unsafe impl Send for ObsFader {}
unsafe impl Sync for ObsFader {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fader_creation() {
        // This is a basic compile-time test
        // Runtime testing would require OBS context initialization
    }
}
