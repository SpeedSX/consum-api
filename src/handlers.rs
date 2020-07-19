use crate::{db::DB, model::*};
use warp::{self, Rejection, Reply};

pub async fn list_orders(db: DB) -> Result<impl Reply, Rejection> {
    db.get_orders()
        .await
        .map(|orders| warp::reply::json(&orders))
        .map_err(crate::problem::from_anyhow)
        .map_err(warp::reject::custom)
}

pub async fn get_order(id: i32, db: DB) -> Result<impl Reply, Rejection> {
    db.get_order(id)
        .await
        // TODO: Return empty content instead of 'null' when NOT_FOUND
        .map(|order| warp::reply::json(&order))
        .map_err(crate::problem::from_anyhow)
        .map_err(warp::reject::custom)
}

pub async fn create_order(order: CreateOrder, db: DB) -> Result<impl Reply, Rejection> {
    db.create_order(order)
        .await
        .map(|order| warp::reply::with_status(warp::reply::json(&order), warp::http::StatusCode::CREATED))
        .map_err(crate::problem::from_anyhow)
        .map_err(warp::reject::custom)
}

pub async fn list_categories(db: DB) -> Result<impl Reply, Rejection> {
    db.get_categories()
        .await
        .map(|cats| warp::reply::json(&cats))
        .map_err(crate::problem::from_anyhow)
        .map_err(warp::reject::custom)
}
