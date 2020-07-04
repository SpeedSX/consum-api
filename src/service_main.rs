use std::{convert::Infallible, env};
use warp::Filter;
use tokio::runtime::Runtime;
use crate::{
    handlers,
    problem,
    configuration::SERVICE_CONFIG, 
    DBPool,
    connection_manager::TiberiusConnectionManager
};
use tiberius::Config;

pub fn run() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "orders=info,service=info");
    }
    pretty_env_logger::init();

    // Create the runtime
    let mut rt = Runtime::new().unwrap(); 

    // Spawn the root task
    rt.block_on(async {
        let manager = TiberiusConnectionManager::new(Config::from_ado_string(SERVICE_CONFIG.get_connection_string()).unwrap()).unwrap();
        let db_pool = bb8::Pool::builder().max_size(SERVICE_CONFIG.get_max_pool()).build_unchecked(manager);
        
        // GET /orders => 200 OK with orders list
        let orders_route = warp::path!("orders")
            .and(with_db(db_pool.clone()))
            .and_then(handlers::list_orders)
            .with(warp::log("orders"))
            .recover(problem::unpack_problem);
    
        info!(target: "service", "Listening on {}", SERVICE_CONFIG.get_addr());

        warp::serve(orders_route)
            //.unstable_pipeline()
            //.run(([127, 0, 0, 1], service_config.get_port()))
            .run(SERVICE_CONFIG.get_addr())
            .await;
    });
}

fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}