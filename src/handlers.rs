use crate::{db::DB, model::*};
use warp::{self, Rejection, Reply, reply, http::StatusCode};

pub async fn list_orders(db: DB) -> Result<impl Reply, Rejection> {
    map_result(
        db.get_orders()
          .await
          .map(|orders| reply::json(&orders)))
}

pub async fn get_order(id: i32, db: DB) -> Result<impl Reply, Rejection> {
    map_result(
        db.get_order(id)
          .await
          .map(|order| reply::json(&order)))
}

pub async fn create_order(order: CreateOrder, db: DB) -> Result<impl Reply, Rejection> {
    map_result(
        db.create_order(order)
          .await
          .map(|order| reply::with_status(reply::json(&order), StatusCode::CREATED)))
}

pub async fn list_categories(db: DB) -> Result<impl Reply, Rejection> {
    map_result(
        db.get_categories()
          .await
          .map(|cats| reply::json(&cats)))
}

pub async fn get_category(id: i32, db: DB) -> Result<impl Reply, Rejection> {
    map_result(
        db.get_category(id)
          .await
          .map(|cat| reply::json(&cat)))
}

pub async fn create_category(cat: CreateCategory, db: DB) -> Result<impl Reply, Rejection> {
    map_result(
        db.create_category(cat)
          .await
          .map(|cat| reply::with_status(reply::json(&cat), StatusCode::CREATED)))
}

fn map_result(result: anyhow::Result<impl Reply>) -> Result<impl Reply, Rejection> {
    result
         .map_err(crate::problem::from_anyhow)
         .map_err(warp::reject::custom)
}
