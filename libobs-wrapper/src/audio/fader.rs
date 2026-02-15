//! Fader controls for managing audio levels.
//!
//! A fader is used to map input values from UI controls to dB and multiplier values
//! that libobs uses to mix audio. The fader internally stores its position as a dB value.

use crate::{
    data::object::ObsObjectTrait, impl_obs_drop, run_with_obs, runtime::ObsRuntime,
    sources::ObsSourceRef, unsafe_send::Sendable, utils::ObsError,
};
use std::sync::Arc;

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
/// This struct is a smart pointer that can be cloned and is thread-safe.
/// It must be created via [`crate::context::ObsContext::fader()`].
///
/// # Example
/// ```no_run
/// use libobs_wrapper::audio::ObsFaderType;
/// use libobs_wrapper::context::ObsContext;
/// use libobs_wrapper::utils::StartupInfo;
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let info = StartupInfo::default();
/// let context = ObsContext::new(info)?;
///
/// // Create a cubic fader via the context
/// let fader = context.fader(ObsFaderType::Cubic)?;
///
/// // Set the level to -6 dB
/// fader.set_db(-6.0);
///
/// // Get the multiplier value for mixing
/// let mul = fader.get_mul();
///
/// // The fader can be cloned
/// let fader_clone = fader.clone();
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct ObsFader {
    inner: Arc<ObsFaderInner>,
}

#[derive(Debug)]
struct ObsFaderInner {
    runtime: ObsRuntime,
    fader: Sendable<*mut libobs::obs_fader_t>,
}

impl ObsFader {
    /// Creates a new fader with the specified type.
    ///
    /// This is internal - users should create faders via `ObsContext::fader()`.
    ///
    /// # Arguments
    /// * `fader_type` - The type of fader curve to use
    /// * `runtime` - The OBS runtime instance
    ///
    /// # Returns
    /// A new `ObsFader` instance, or an error if creation failed
    pub(crate) fn new(fader_type: ObsFaderType, runtime: ObsRuntime) -> Result<Self, ObsError> {
        let fader_type_val = fader_type as u32;

        let fader_ptr = run_with_obs!(runtime, move || unsafe {
            Sendable(libobs::obs_fader_create(fader_type_val))
        })?;

        if fader_ptr.0.is_null() {
            return Err(ObsError::NullPointer(Some(
                "Failed to create fader".to_string(),
            )));
        }

        Ok(Self {
            inner: Arc::new(ObsFaderInner {
                runtime,
                fader: fader_ptr,
            }),
        })
    }

    /// Sets the fader dB value.
    ///
    /// # Arguments
    /// * `db` - The new dB value to set
    ///
    /// # Returns
    /// `true` if the value was set without clamping, `false` if it was clamped to limits
    pub fn set_db(&self, db: f32) -> bool {
        unsafe { libobs::obs_fader_set_db(self.inner.fader.0, db) }
    }

    /// Gets the current fader dB value.
    pub fn get_db(&self) -> f32 {
        unsafe { libobs::obs_fader_get_db(self.inner.fader.0) }
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
        unsafe { libobs::obs_fader_set_deflection(self.inner.fader.0, def) }
    }

    /// Gets the current fader deflection value.
    pub fn get_deflection(&self) -> f32 {
        unsafe { libobs::obs_fader_get_deflection(self.inner.fader.0) }
    }

    /// Sets the fader value from a multiplier.
    ///
    /// # Arguments
    /// * `mul` - The multiplier value to set
    ///
    /// # Returns
    /// `true` if the value was set without clamping, `false` if it was clamped to limits
    pub fn set_mul(&self, mul: f32) -> bool {
        unsafe { libobs::obs_fader_set_mul(self.inner.fader.0, mul) }
    }

    /// Gets the current fader multiplier value.
    ///
    /// This is the actual multiplier that will be applied to audio samples.
    pub fn get_mul(&self) -> f32 {
        unsafe { libobs::obs_fader_get_mul(self.inner.fader.0) }
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
        unsafe { libobs::obs_fader_attach_source(self.inner.fader.0, source.as_ptr().get_ptr()) }
    }

    /// Detaches the fader from its currently attached source.
    pub fn detach_source(&self) {
        unsafe { libobs::obs_fader_detach_source(self.inner.fader.0) }
    }

    /// Returns the raw pointer to the fader.
    ///
    /// # Safety
    /// The caller must ensure the pointer is used safely and doesn't outlive the fader.
    pub fn as_ptr(&self) -> *mut libobs::obs_fader_t {
        self.inner.fader.0
    }
}

impl_obs_drop!(ObsFaderInner, (fader), move || {
    unsafe {
        libobs::obs_fader_destroy(fader.0);
    }
});

#[cfg(test)]
mod tests {
    #[test]
    fn test_fader_creation() {
        // This is a basic compile-time test
        // Runtime testing would require OBS context initialization
    }
}
