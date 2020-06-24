use warp::Filter;
use std::env;
use once_cell::sync::Lazy;
#[macro_use] extern crate log;

static CONN_STR: Lazy<String> = Lazy::new(|| {
    env::var("CONSUM_CONNECTION_STRING")
        .unwrap_or_else(|_| "server=tcp:localhost\\SQLEXPRESS,1433;User=sa;Password=sas;Database=Consum".to_owned())
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
        .run(([127, 0, 0, 1], 3030))
        .await;
}

mod handlers {
    use warp::{self, Rejection, Reply};

    pub async fn list_orders() -> Result<impl Reply, Rejection> {
        super::service::get_orders()
            .await
            .map(|orders| warp::reply::json(&orders))
            .map_err(super::problem::from_anyhow)
            .map_err(|e| warp::reject::custom(e))
    }
}

mod model {
    use serde::Serialize;

    #[derive(Debug, Serialize)]
    pub struct Order {
        pub consId: i32,
        pub accountNum: String,
        pub accountDate: String
    }
}

mod service {
    use anyhow::Result;
    use tokio::net::TcpStream;
    use tiberius::{Config, Client, Row};
    use tokio_util::compat::Tokio02AsyncWriteCompatExt;
    use super::CONN_STR;
    use super::model::Order;

    pub async fn get_orders() -> Result<Vec<Order>> {
        let config = Config::from_ado_string(&CONN_STR)?;

        let tcp = TcpStream::connect(config.get_addr()).await?;
        tcp.set_nodelay(true)?;
    
        let mut client = Client::connect(config, tcp.compat_write()).await?;
    
        let stream = client.simple_query("SELECT * from ConsOrders").await?;
        let rows: Vec<Row> = stream.into_first_result().await?;
        
        let orders: Vec<Order> = rows
            .into_iter()
            .map(|r| {
                let account_num: &str = r.get("AccountNum").unwrap_or("");
                Order { 
                    consId: r.get("ConsID").unwrap_or(0),
                    accountNum: account_num.to_owned(),
                    accountDate: "".to_string()
                }})
            .collect();

        info!(target: "orders", "Orders count = {}", orders.len());

        Ok(orders)
    }
}

mod problem {
    use http_api_problem::HttpApiProblem;
    use warp::{
        self,
        http::{self, StatusCode},
        Rejection, Reply,
    };

    pub fn from_anyhow(e: anyhow::Error) -> HttpApiProblem {
        let e = match e.downcast::<HttpApiProblem>() {
            Ok(problem) => return problem,
            Err(e) => e,
        };
        HttpApiProblem::new(format!("Internal Server Error\n{:?}", e))
            .set_status(warp::http::StatusCode::INTERNAL_SERVER_ERROR)
    }

    pub async fn unpack_problem(rejection: Rejection) -> Result<impl Reply, Rejection> {
        if let Some(problem) = rejection.find::<HttpApiProblem>() {
            let code = problem.status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

            let reply = warp::reply::json(problem);
            let reply = warp::reply::with_status(reply, code);
            let reply = warp::reply::with_header(
                reply,
                http::header::CONTENT_TYPE,
                http_api_problem::PROBLEM_JSON_MEDIA_TYPE,
            );

            return Ok(reply);
        }

        Err(rejection)
    }
}