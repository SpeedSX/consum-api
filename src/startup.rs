use std::{convert::Infallible, env};
use warp::Filter;
use tokio::{
    runtime::Runtime, 
    sync::oneshot::{self, Receiver}
};
use tiberius::{Config};
use http_api_problem::HttpApiProblem;
use crate::{
    handlers,
    problem,
    configuration, 
    DBPool,
    connection_manager::TiberiusConnectionManager,
    db::DB, 
    url_part_utf8_string::UrlPartUtf8String,
    model::*, 
    auth, 
};
use chrono::DateTime;
use configuration::Configuration;

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
        let config = configuration::get();
        let manager = TiberiusConnectionManager::new(Config::from_ado_string(config.connection_string()).unwrap()).unwrap();
        let db_pool = bb8::Pool::builder().max_size(config.max_pool()).build_unchecked(manager);
        
        //test(db_pool.clone()).await;
        
        let api = api(db_pool)
            .with(warp::log("api"))
            .recover(problem::unpack_problem);
    
        info!(target: "service", "Listening on {}", config.addr());

        if generate_auth_token(config).is_none() {
            return;
        }

        let (_addr, server) = warp::serve(api)
           .bind_with_graceful_shutdown(config.addr(), async {
              shutdown_rx.await.ok();
           });
         server.await;
    });
}

// async fn test(pool: DBPool) {
//     let mut con = pool.get().await.unwrap();
//     let result = con.simple_query("select * from Test").await.unwrap();
//     let rows: Vec<Row> = result.into_first_result().await.unwrap();
//     info!("{:?}", rows);
//     let a: Option<i32> = rows[0].try_get("value").unwrap();
//     info!("a = {:?}", a);
//     let b: Option<i32> = rows[1].try_get("value").unwrap();
//     info!("b = {:?}", b);
// }

fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || DB::new(db_pool.clone()))
}

// Generate and show token
fn generate_auth_token(config: &Configuration) -> Option<String> {
    let exp = DateTime::parse_from_rfc3339("2030-01-01T01:01:01.00Z").unwrap();
    let token = auth::try_encode_token_exp(config.jwt_secret(), "1", exp.into());

    token.map(|v| {
        info!(target: "service", "Auth token {:?}", v);
        v
    })
    .map_err(|error| {
        error!(target: "service", "Error encoding auth token: {}", error);
        error
    })
    .ok()

    // TODO: Not sure which way looks better, this works too
    // match token {
    //     Ok(token) => {
    //         info!(target: "service", "Auth token {:?}", token);
    //         return Some(token);
    //     },
    //     Err(error) => {
    //         error!(target: "service", "Error encoding auth token: {}", error);
    //         return None;
    //     } 
    // }
}

// API key verification
fn auth_check() -> impl Filter<Extract = (User,), Error = warp::Rejection> + Copy {
    warp::query().and_then(|key: ApiKey| async move {
        let jwt_secret = configuration::get().jwt_secret();
        let claims = auth::decode_token(jwt_secret, &key.api_key);
        match claims {
            Ok(claims) => Ok(User { id: claims.user_id().to_owned() }),
            Err(err) => Err(warp::reject::custom(
                HttpApiProblem::new(format!("Invalid API key: {:?}", err.kind()))
                    .set_status(warp::http::StatusCode::UNAUTHORIZED)))
        }
    })
}

// Endpoints

pub fn orders(
    db: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("orders")
        .and(warp::get())
        .and(auth_check())
        .and(with_db(db))
        .and_then(handlers::list_orders)
}

pub fn create_orders_view(
    db: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("orders" / "views")
        .and(warp::post())
        .and(warp::body::json())
        .and(auth_check())
        .and(with_db(db))
        .and_then(handlers::list_orders_filtered)
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
        .and(auth_check())
        .and(with_db(db))
        .and_then(handlers::create_order)
}

pub fn categories(
    db: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("categories")
        .and(warp::get())
        .and(auth_check())
        .and(with_db(db))
        .and_then(handlers::list_categories)
}

pub fn category(
    db: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("categories" / i32)
        .and(warp::get())
        .and(auth_check())
        .and(with_db(db))
        .and_then(handlers::get_category)
}

pub fn create_category(
    db: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("categories")
        .and(warp::post())
        .and(warp::body::json())
        .and(auth_check())
        .and(with_db(db))
        .and_then(handlers::create_category)
}

pub fn delete_category(
    db: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("categories" / i32)
        .and(warp::delete())
        .and(auth_check())
        .and(with_db(db))
        .and_then(handlers::delete_category)
}

pub fn supplier_by_id(
    db: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("suppliers" / i32)
        .and(warp::get())
        .and(auth_check())
        .and(with_db(db))
        .and_then(handlers::get_supplier_by_id)
}

pub fn supplier_by_name(
    db: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("suppliers" / "name" / UrlPartUtf8String)
        .and(warp::get())
        .and(auth_check())
        .and(with_db(db))
        .and_then(handlers::get_supplier_by_name)
}

pub fn create_supplier(
    db: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("suppliers")
        .and(warp::post())
        .and(warp::body::json())
        .and(auth_check())
        .and(with_db(db))
        .and_then(handlers::create_supplier)
}

// Aggregate all endpoints

pub fn api(
    db: DBPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    orders(db.clone())
        .or(create_orders_view(db.clone()))
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

    if configuration::get().stdout_enabled() {
        logger = logger.chain(std::io::stdout());
    }

    if let Some(path) = configuration::get().log_path() {
        println!("Logging to file {}", path);
        logger = logger.chain(fern::log_file(path)?)
    }

    logger.apply()?;

    Ok(())
}