mod problem;
mod service;
mod handlers;
mod model;

use warp::Filter;
use std::env;
use once_cell::sync::Lazy;
#[macro_use] extern crate log;

static CONN_STR: Lazy<String> = Lazy::new(|| {
    env::var("CONSUM_CONNECTION_STRING")
        .unwrap_or_else(|_| "server=tcp:localhost\\SQLEXPRESS,1433;User=sa;Password=sas;Database=Consum".to_owned())
});

static DEFAULT_PORT: u16 = 3030;

static PORT: Lazy<u16> = Lazy::new(|| {
    env::var("CONSUM_PORT")
        .map(|s| s.parse::<u16>().unwrap_or(DEFAULT_PORT))
        .unwrap_or_else(|_| DEFAULT_PORT)
});

#[cfg(not(all(windows, feature = "sql-browser-tokio")))]
#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "orders=info");
    }
    pretty_env_logger::init();

    // GET /orders => 200 OK with orders list
    let orders_route = warp::path!("orders")
        .and_then(handlers::list_orders)
        .with(warp::log("orders"))
        .recover(problem::unpack_problem);

    warp::serve(orders_route)
        .run(([127, 0, 0, 1], *PORT))
        .await;
}
