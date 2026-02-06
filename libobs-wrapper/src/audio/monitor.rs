//! Audio monitoring controls for sources.
//!
//! Audio monitoring allows you to monitor (listen to) a source's audio output
//! on a specific audio device, independent of the main output.

use crate::{data::object::ObsObjectTrait, sources::ObsSourceRef};

/// Type of audio monitoring for a source.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObsMonitoringType {
    /// No monitoring - source audio is only sent to outputs.
    None = libobs::obs_monitoring_type_OBS_MONITORING_TYPE_NONE,

    /// Monitor only - source audio is sent to the monitoring device but not to outputs.
    /// Useful for preview/cue functionality.
    MonitorOnly = libobs::obs_monitoring_type_OBS_MONITORING_TYPE_MONITOR_ONLY,

    /// Monitor and output - source audio is sent to both the monitoring device and outputs.
    /// This is the typical mode for hearing what's being streamed/recorded.
    MonitorAndOutput = libobs::obs_monitoring_type_OBS_MONITORING_TYPE_MONITOR_AND_OUTPUT,
}

impl From<ObsMonitoringType> for u32 {
    fn from(t: ObsMonitoringType) -> u32 {
        t as u32
    }
}

impl From<u32> for ObsMonitoringType {
    fn from(val: u32) -> Self {
        match val {
            libobs::obs_monitoring_type_OBS_MONITORING_TYPE_NONE => ObsMonitoringType::None,
            libobs::obs_monitoring_type_OBS_MONITORING_TYPE_MONITOR_ONLY => {
                ObsMonitoringType::MonitorOnly
            }
            libobs::obs_monitoring_type_OBS_MONITORING_TYPE_MONITOR_AND_OUTPUT => {
                ObsMonitoringType::MonitorAndOutput
            }
            _ => ObsMonitoringType::None,
        }
    }
}

/// Extension trait for adding audio monitoring methods to sources.
pub trait ObsSourceAudioMonitoring {
    /// Sets the audio monitoring type for this source.
    ///
    /// This controls whether and how the source's audio is sent to a monitoring device.
    /// The actual monitoring device is configured globally in OBS settings.
    ///
    /// # Arguments
    /// * `monitoring_type` - The type of monitoring to use
    ///
    /// # Example
    /// ```no_run
    /// use libobs_wrapper::audio::{ObsMonitoringType, ObsSourceAudioMonitoring};
    /// # use libobs_wrapper::sources::ObsSourceRef;
    /// # fn example(source: &ObsSourceRef) {
    /// // Enable monitoring and output for a source
    /// source.set_monitoring_type(ObsMonitoringType::MonitorAndOutput);
    /// # }
    /// ```
    fn set_monitoring_type(&self, monitoring_type: ObsMonitoringType);

    /// Gets the current audio monitoring type for this source.
    ///
    /// # Returns
    /// The current monitoring type
    fn get_monitoring_type(&self) -> ObsMonitoringType;
}

impl ObsSourceAudioMonitoring for ObsSourceRef {
    fn set_monitoring_type(&self, monitoring_type: ObsMonitoringType) {
        unsafe {
            libobs::obs_source_set_monitoring_type(self.as_ptr().get_ptr(), monitoring_type as u32);
        }
    }

    fn get_monitoring_type(&self) -> ObsMonitoringType {
        let val = unsafe { libobs::obs_source_get_monitoring_type(self.as_ptr().get_ptr()) };
        ObsMonitoringType::from(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitoring_type_conversion() {
        assert_eq!(
            ObsMonitoringType::None as u32,
            libobs::obs_monitoring_type_OBS_MONITORING_TYPE_NONE
        );
        assert_eq!(
            ObsMonitoringType::MonitorOnly as u32,
            libobs::obs_monitoring_type_OBS_MONITORING_TYPE_MONITOR_ONLY
        );
        assert_eq!(
            ObsMonitoringType::MonitorAndOutput as u32,
            libobs::obs_monitoring_type_OBS_MONITORING_TYPE_MONITOR_AND_OUTPUT
        );
    }
}
