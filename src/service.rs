use std::env;
use warp::Filter;
use tokio::runtime::Runtime;
use super::configuration::service_config;
use super::handlers;
use super::problem;

pub fn run() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "orders=info,service=info");
    }
    pretty_env_logger::init();

    // GET /orders => 200 OK with orders list
    let orders_route = warp::path!("orders")
        .and_then(handlers::list_orders)
        .with(warp::log("orders"))
        .recover(problem::unpack_problem);

    // Create the runtime
    let mut rt = Runtime::new().unwrap(); 

    // Spawn the root task
    rt.block_on(async {
        info!(target: "service", "Listening on 127.0.0.1:{}", service_config.get_port());

        warp::serve(orders_route)
            .run(([127, 0, 0, 1], service_config.get_port()))
            .await;
    });
}