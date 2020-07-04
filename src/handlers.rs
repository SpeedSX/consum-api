use crate::{db, DBPool};
use warp::{self, Rejection, Reply};

pub async fn list_orders(db_pool: DBPool) -> Result<impl Reply, Rejection> {
    db::get_orders(db_pool)
        .await
        .map(|orders| warp::reply::json(&orders))
        .map_err(crate::problem::from_anyhow)
        .map_err(|e| warp::reject::custom(e))
}


pub async fn list_categories(db_pool: DBPool) -> Result<impl Reply, Rejection> {
    db::get_categories(db_pool)
        .await
        .map(|cats| warp::reply::json(&cats))
        .map_err(crate::problem::from_anyhow)
        .map_err(|e| warp::reject::custom(e))
}
