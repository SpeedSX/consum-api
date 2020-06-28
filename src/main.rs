mod problem;
mod service;
mod repository;
mod handlers;
mod model;
mod configuration;

#[cfg(feature = "run-windows-service")]
mod windows_service_main;

#[macro_use] extern crate log;

#[cfg(feature = "run-windows-service")]
#[macro_use] extern crate windows_service;

#[cfg(feature = "run-windows-service")]
fn main() -> windows_service::Result<()> {
    windows_service_main::run()
}

#[cfg(not(feature="run-windows-service"))]
fn main() {
    service::run_service();
}
