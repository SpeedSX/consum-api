use warp::{self, Rejection, Reply};

pub async fn list_orders() -> Result<impl Reply, Rejection> {
    super::repository::get_orders()
        .await
        .map(|orders| warp::reply::json(&orders))
        .map_err(super::problem::from_anyhow)
        .map_err(|e| warp::reject::custom(e))
}
