mod startup;
mod configuration;
mod db;
mod handlers;
mod model;
mod connection_manager;
mod serialization;
mod problem;

use connection_manager::TiberiusConnectionManager;

type DBPool = bb8::Pool<TiberiusConnectionManager>;

#[cfg(feature = "run-windows-service")]
mod windows_service_main;

#[macro_use] extern crate log;

//#[cfg(feature = "run-windows-service")]
//#[macro_use] extern crate windows_service;

#[cfg(feature = "run-windows-service")]
fn main() -> windows_service::Result<()> {
    windows_service_main::run()
}

#[cfg(not(feature="run-windows-service"))]
fn main() {
    startup::run();
}
