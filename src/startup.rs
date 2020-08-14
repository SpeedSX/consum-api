use std::{convert::Infallible, env};
use warp::Filter;
use tokio::{
    runtime::Runtime, 
    sync::oneshot::{self, Receiver}
};
use tiberius::Config;
use http_api_problem::HttpApiProblem;
use crate::{
    handlers,
    problem,
    configuration::SERVICE_CONFIG, 
    DBPool,
    connection_manager::TiberiusConnectionManager,
    db::DB, 
    url_part_utf8_string::UrlPartUtf8String,
    model::*, 
    auth, 
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
        
          let api = api(db_pool)
            .with(warp::log("api"))
            .recover(problem::unpack_problem);
    
        info!(target: "service", "Listening on {}", SERVICE_CONFIG.get_addr());
        info!("Auth token {:?}", auth::encode_token(SERVICE_CONFIG.get_jwt_secret(), "1"));

        let (_addr, server) = warp::serve(api)
           .bind_with_graceful_shutdown(SERVICE_CONFIG.get_addr(), async {
              shutdown_rx.await.ok();
           });
         server.await;
    });
}

fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || DB::new(db_pool.clone()))
}

// Endpoints

pub fn orders(
    db: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("orders")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::list_orders)
}

fn auth_check() -> impl Filter<Extract = (User,), Error = warp::Rejection> + Copy {
    warp::query().and_then(|key: ApiKey| async move {
        let jwt_secret = SERVICE_CONFIG.get_jwt_secret();
        // API key verification
        let claims = auth::decode_token(jwt_secret, &key.api_key);
        match claims {
            Ok(claims) => Ok(User { id: claims.user_id().to_owned() }),
            Err(err) => Err(warp::reject::custom(
                HttpApiProblem::new(format!("Invalid API key: {:?}", err.kind()))
                    .set_status(warp::http::StatusCode::UNAUTHORIZED)))
        }
    })
}

pub fn order(
    db: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("orders" / i32)
        .and(warp::get())
        .and(auth_check())
        .and(with_db(db))
        .and_then(handlers::get_order)
}

pub fn create_order(
    db: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("orders")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(handlers::create_order)
}

pub fn categories(
    db: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("categories")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::list_categories)
}

pub fn category(
    db: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("categories" / i32)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::get_category)
}

pub fn create_category(
    db: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("categories")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(handlers::create_category)
}

pub fn delete_category(
    db: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("categories" / i32)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(handlers::delete_category)
}

pub fn supplier_by_id(
    db: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("suppliers" / i32)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::get_supplier_by_id)
}

pub fn supplier_by_name(
    db: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("suppliers" / "name" / UrlPartUtf8String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::get_supplier_by_name)
}

// Aggregate all endpoints

pub fn create_supplier(
    db: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("suppliers")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(handlers::create_supplier)
}

pub fn api(
    db: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    orders(db.clone())
        .or(order(db.clone()))
        .or(create_order(db.clone()))
        .or(categories(db.clone()))
        .or(category(db.clone()))
        .or(create_category(db.clone()))
        .or(delete_category(db.clone()))
        .or(supplier_by_id(db.clone()))
        .or(supplier_by_name(db.clone()))
        .or(create_supplier(db.clone()))
    // one of these clone()'s is not required but left for consistency
}

fn setup_logger() -> Result<(), fern::InitError> {
    let mut logger = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug);

    if SERVICE_CONFIG.stdout_enabled() {
        logger = logger.chain(std::io::stdout());
    }

    if let Some(path) = SERVICE_CONFIG.get_log_path() {
        println!("Logging to file {}", path);
        logger = logger.chain(fern::log_file(path)?)
    }

    logger.apply()?;

    Ok(())
}