use env_logger::Env;
use libobs_simple::output::simple::ObsContextSimpleExt;
use libobs_wrapper::{
    context::ObsContext,
    data::output::ObsOutputRef,
    encoders::{ObsContextEncoders, ObsVideoEncoderType},
    utils::{AudioEncoderInfo, ObsString, OutputInfo, StartupInfo},
};

/// The string returned is the name of the obs output
#[allow(dead_code)]
pub fn initialize_obs<T: Into<ObsString> + Send + Sync>(rec_file: T) -> (ObsContext, ObsOutputRef) {
    let _ = env_logger::Builder::from_env(Env::default().default_filter_or("debug"))
        .is_test(true)
        .try_init();

    #[allow(unused_mut)]
    let mut context = ObsContext::new(StartupInfo::default()).unwrap();

    // Set up output to ./recording.mp4
    let mut output_settings = context.data().unwrap();
    output_settings.set_string("path", rec_file).unwrap();

    let mut output = context
        .simple_output_builder("test_obs_output", rec_file.to_string())
        .build()
        .unwrap();

    (context, output)
}
