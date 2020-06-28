use std::{ffi::OsString};
use windows_service::{service_dispatcher, Result};
use super::service;

define_windows_service!(ffi_service_main, my_service_main);

fn my_service_main(_arguments: Vec<OsString>) {
    // The entry point where execution will start on a background thread after a call to
    // `service_dispatcher::start` from `main`.
    service::run_service();
}

const SERVICE_NAME: &str = "PolyConsService";

//#[cfg(not(all(windows, feature = "sql-browser-tokio")))]
pub fn run() -> Result<()> {
    // Register generated `ffi_service_main` with the system and start the service, blocking
    // this thread until the service is stopped.
    service_dispatcher::start(SERVICE_NAME, ffi_service_main)
}