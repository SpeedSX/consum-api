mod auth;
mod configuration;
mod connection_manager;
mod db;
mod errors;
mod handlers;
mod http_compat;
mod model;
mod problem;
mod startup;
mod url_part_utf8_string;

use connection_manager::TiberiusConnection;

type DBPool = bb8::Pool<TiberiusConnection>;

#[cfg(feature = "run-windows-service")]
mod windows_service_main;

#[macro_use]
extern crate log;

#[cfg(feature = "run-windows-service")]
fn main() -> windows_service::Result<()> {
    windows_service_main::run()
}

#[cfg(not(feature = "run-windows-service"))]
fn main() {
    startup::run();
}
