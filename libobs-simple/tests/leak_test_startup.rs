use env_logger::Env;
use libobs_wrapper::{context::ObsContext, utils::StartupInfo};
use serial_test::serial;

/// Stage 1: Initialize OBS with basic configuration
#[test]
#[serial]
pub fn test_startup() {
    let _ = env_logger::Builder::from_env(Env::default().default_filter_or("debug"))
        .is_test(true)
        .try_init();

    // Start the OBS context
    #[allow(unused_mut)]
    let mut startup_info = StartupInfo::default();

    // Create OBS context
    let _context = ObsContext::new(startup_info).unwrap();
    // Context will be dropped here, testing for memory leaks
}
