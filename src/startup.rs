use std::{convert::Infallible, env};
use warp::Filter;
use tokio::{runtime::Runtime, sync::oneshot::{self, Receiver}};
use tiberius::Config;
use crate::{
    handlers,
    problem,
    configuration::SERVICE_CONFIG, 
    DBPool,
    connection_manager::TiberiusConnectionManager,
    db::DB
};

pub fn run() {
    let (_tx, rx) = oneshot::channel::<()>();
    run_with_graceful_shutdown(rx);
}

pub fn run_with_graceful_shutdown<T>(shutdown_rx: Receiver<T>) where T: Send + 'static {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "api=info,service=info");
    }
    //pretty_env_logger::init();
    setup_logger().ok();

    // Create the runtime
    let mut rt = Runtime::new().unwrap(); 
    
    // Spawn the root task
    rt.block_on(async {
        let manager = TiberiusConnectionManager::new(Config::from_ado_string(SERVICE_CONFIG.get_connection_string()).unwrap()).unwrap();
        let db_pool = bb8::Pool::builder().max_size(SERVICE_CONFIG.get_max_pool()).build_unchecked(manager);
        
        // GET /orders => 200 OK with orders list
        let api = api(db_pool)
            .with(warp::log("api"))
            .recover(problem::unpack_problem);
    
        info!(target: "service", "Listening on {}", SERVICE_CONFIG.get_addr());

        let (_addr, server) = warp::serve(api)
            //.unstable_pipeline()
            //.run(([127, 0, 0, 1], service_config.get_port()))
            //.run(SERVICE_CONFIG.get_addr())
         .bind_with_graceful_shutdown(SERVICE_CONFIG.get_addr(), async {
            shutdown_rx.await.ok();
         });
         server.await;
    });
}

fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || DB::new(db_pool.clone()))
}

pub fn orders(
    db: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("orders")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::list_orders)
}


pub fn categories(
    db: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("categories")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::list_categories)
}

pub fn api(
    db: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    orders(db.clone())
        .or(categories(db))
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        //.chain(fern::log_file("h:\\Projects\\consum-api\\output.log")?)
        .apply()?;
    Ok(())
}