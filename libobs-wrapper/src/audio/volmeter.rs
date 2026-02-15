//! Volume meter (volmeter) for monitoring audio levels.
//!
//! A volume meter monitors audio levels from a source and prepares the data
//! for display in a GUI, automatically taking source volume into account.

use crate::{
    data::object::ObsObjectTrait, impl_obs_drop, run_with_obs, runtime::ObsRuntime,
    sources::ObsSourceRef, unsafe_send::Sendable, utils::ObsError,
};
use std::sync::Arc;

/// Type of peak meter to use for level measurement.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObsPeakMeterType {
    /// A simple peak meter measuring the maximum of all samples.
    /// Common but not very accurate for further audio processing.
    SamplePeak = libobs::obs_peak_meter_type_SAMPLE_PEAK_METER,

    /// An accurate peak meter measuring the maximum of inter-samples.
    /// More computationally intensive (4x oversampling) but provides
    /// true peak accuracy to +/- 0.5 dB.
    TruePeak = libobs::obs_peak_meter_type_TRUE_PEAK_METER,
}

impl From<ObsPeakMeterType> for u32 {
    fn from(t: ObsPeakMeterType) -> u32 {
        t as u32
    }
}

/// Maximum number of audio channels supported by libobs.
pub const MAX_AUDIO_CHANNELS: usize = libobs::MAX_AUDIO_CHANNELS as usize;

/// A volume meter for monitoring audio source levels.
///
/// The volume meter attaches to a source and monitors its audio levels,
/// providing magnitude, peak, and input peak values for each channel.
/// It automatically maps levels to the range [0.0, 1.0] for GUI display.
///
/// This struct is a smart pointer that can be cloned and is thread-safe.
/// It must be created via [`crate::context::ObsContext::volmeter()`].
///
/// # Example
/// ```no_run
/// use libobs_wrapper::audio::{ObsPeakMeterType, ObsFaderType};
/// use libobs_wrapper::context::ObsContext;
/// use libobs_wrapper::utils::StartupInfo;
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let info = StartupInfo::default();
/// let context = ObsContext::new(info)?;
///
/// // Create a volume meter with IEC fader mapping via the context
/// let volmeter = context.volmeter(ObsFaderType::IEC)?;
///
/// // Set to use true peak metering for accuracy
/// volmeter.set_peak_meter_type(ObsPeakMeterType::TruePeak);
///
/// // Get number of channels
/// let channels = volmeter.get_nr_channels();
///
/// // The volmeter can be cloned
/// let volmeter_clone = volmeter.clone();
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct ObsVolmeter {
    inner: Arc<ObsVolmeterInner>,
}

#[derive(Debug)]
struct ObsVolmeterInner {
    runtime: ObsRuntime,
    volmeter: Sendable<*mut libobs::obs_volmeter_t>,
}

impl ObsVolmeter {
    /// Creates a new volume meter with the specified fader type for level mapping.
    ///
    /// This is internal - users should create volmeters via `ObsContext::volmeter()`.
    ///
    /// # Arguments
    /// * `fader_type` - The fader type to use for mapping levels to display values
    /// * `runtime` - The OBS runtime instance
    ///
    /// # Returns
    /// A new `ObsVolmeter` instance, or an error if creation failed
    pub(crate) fn new(
        fader_type: crate::audio::ObsFaderType,
        runtime: ObsRuntime,
    ) -> Result<Self, ObsError> {
        let fader_type_val = fader_type as u32;

        let volmeter_ptr = run_with_obs!(runtime, move || unsafe {
            Sendable(libobs::obs_volmeter_create(fader_type_val))
        })?;

        if volmeter_ptr.0.is_null() {
            return Err(ObsError::NullPointer(Some(
                "Failed to create volmeter".to_string(),
            )));
        }

        Ok(Self {
            inner: Arc::new(ObsVolmeterInner {
                runtime,
                volmeter: volmeter_ptr,
            }),
        })
    }

    /// Attaches the volume meter to a source.
    ///
    /// When attached, the volume meter starts listening to audio updates from the source
    /// and processes the data before emitting callbacks.
    ///
    /// # Arguments
    /// * `source` - The source to attach to
    ///
    /// # Returns
    /// `true` if attachment succeeded, `false` otherwise
    pub fn attach_source(&self, source: &ObsSourceRef) -> bool {
        unsafe {
            libobs::obs_volmeter_attach_source(self.inner.volmeter.0, source.as_ptr().get_ptr())
        }
    }

    /// Detaches the volume meter from its currently attached source.
    pub fn detach_source(&self) {
        unsafe { libobs::obs_volmeter_detach_source(self.inner.volmeter.0) }
    }

    /// Sets the peak meter type.
    ///
    /// # Arguments
    /// * `peak_meter_type` - The type of peak metering to use
    pub fn set_peak_meter_type(&self, peak_meter_type: ObsPeakMeterType) {
        unsafe {
            libobs::obs_volmeter_set_peak_meter_type(self.inner.volmeter.0, peak_meter_type as u32)
        }
    }

    /// Gets the number of audio channels configured for the attached source.
    ///
    /// # Returns
    /// The number of channels, or 0 if no source is attached
    pub fn get_nr_channels(&self) -> i32 {
        unsafe { libobs::obs_volmeter_get_nr_channels(self.inner.volmeter.0) }
    }

    /// Returns the raw pointer to the volmeter.
    ///
    /// # Safety
    /// The caller must ensure the pointer is used safely and doesn't outlive the volmeter.
    pub fn as_ptr(&self) -> *mut libobs::obs_volmeter_t {
        self.inner.volmeter.0
    }
}

impl_obs_drop!(ObsVolmeterInner, (volmeter), move || {
    unsafe {
        libobs::obs_volmeter_destroy(volmeter.0);
    }
});

#[cfg(test)]
mod tests {
    #[test]
    fn test_volmeter_creation() {
        // This is a basic compile-time test
        // Runtime testing would require OBS context initialization
    }
}
