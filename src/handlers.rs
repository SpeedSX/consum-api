use crate::db::DB;
use warp::{self, Rejection, Reply};

pub async fn list_orders(db: DB) -> Result<impl Reply, Rejection> {
    db.get_orders()
        .await
        .map(|orders| warp::reply::json(&orders))
        .map_err(crate::problem::from_anyhow)
        .map_err(|e| warp::reject::custom(e))
}


pub async fn list_categories(db: DB) -> Result<impl Reply, Rejection> {
    db.get_categories()
        .await
        .map(|cats| warp::reply::json(&cats))
        .map_err(crate::problem::from_anyhow)
        .map_err(|e| warp::reject::custom(e))
}
