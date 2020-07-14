use crate::{db::DB, model::*};
use warp::{self, Rejection, Reply};

pub async fn list_orders(db: DB) -> Result<impl Reply, Rejection> {
    db.get_orders()
        .await
        .map(|orders| warp::reply::json(&orders))
        .map_err(crate::problem::from_anyhow)
        .map_err(|e| warp::reject::custom(e))
}

pub async fn get_order(id: i32, db: DB) -> Result<impl Reply, Rejection> {
    db.get_order(id)
        .await
        // TODO: Return empty content instead of 'null' when NOT_FOUND
        .map(|order| order.map_or_else(
            || warp::reply::with_status(warp::reply::json(&()), warp::http::StatusCode::NOT_FOUND),
            |v| warp::reply::with_status(warp::reply::json(&v), warp::http::StatusCode::OK))
        )
        .map_err(crate::problem::from_anyhow)
        .map_err(|e| warp::reject::custom(e))
}

pub async fn create_order(order: CreateOrder, db: DB) -> Result<impl Reply, Rejection> {
    db.create_order(order)
        .await
        // TODO: Return empty content instead of 'null' when NOT_FOUND
        .map(|order| order.map_or_else(
            || warp::reply::with_status(warp::reply::json(&()), warp::http::StatusCode::NOT_FOUND),
            |v| warp::reply::with_status(warp::reply::json(&v), warp::http::StatusCode::CREATED))
        )
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
