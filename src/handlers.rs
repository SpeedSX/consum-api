use crate::{
    db::DB,
    model::*,
    url_part_utf8_string::UrlPartUtf8String
};
use anyhow::Result;
use warp::{self, Rejection, Reply, reply, http::StatusCode};

pub async fn list_orders(_user: User, db: DB) -> Result<impl Reply, Rejection> {
    map_result(
        db.get_orders()
          .await
          .map(|orders| reply::json(&orders)))
}

pub async fn get_order(id: i32, _user: User, db: DB) -> Result<impl Reply, Rejection> {
    map_result(
        db.get_order(id)
          .await
          .map(|order| reply::json(&order)))
}

pub async fn create_order(order: CreateOrder, _user: User, db: DB) -> Result<impl Reply, Rejection> {
    map_result(
        db.create_order(order)
          .await
          .map(|order| reply::with_status(reply::json(&order), StatusCode::CREATED)))
}

pub async fn list_categories(_user: User, db: DB) -> Result<impl Reply, Rejection> {
    map_result(
        db.get_categories()
          .await
          .map(|cats| reply::json(&cats)))
}

pub async fn get_category(id: i32, _user: User, db: DB) -> Result<impl Reply, Rejection> {
    map_result(
        db.get_category(id)
          .await
          .map(|cat| reply::json(&cat)))
}

pub async fn create_category(cat: CreateCategory, _user: User, db: DB) -> Result<impl Reply, Rejection> {
    map_result(
        db.create_category(cat)
          .await
          .map(|cat| reply::with_status(reply::json(&cat), StatusCode::CREATED)))
}

pub async fn delete_category(id: i32, _user: User, db: DB) -> Result<impl Reply, Rejection> {
    map_result(
        db.delete_category(id)
          .await
          .map(|()| reply::reply()))
}

pub async fn get_supplier_by_id(id: i32, _user: User, db: DB) -> Result<impl Reply, Rejection> {
    map_result(
        db.get_supplier_by_id(id)
          .await
          .map(|supplier| reply::json(&supplier)))
}

pub async fn get_supplier_by_name(name: UrlPartUtf8String, _user: User, db: DB) -> Result<impl Reply, Rejection> {
    map_result(
        db.get_supplier_by_name(name.to_string())
          .await
          .map(|supplier| reply::json(&supplier)))
}

pub async fn create_supplier(supplier: CreateSupplier, _user: User, db: DB) -> Result<impl Reply, Rejection> {
    map_result(
        db.create_supplier(supplier)
          .await
          .map(|supplier| reply::with_status(reply::json(&supplier), StatusCode::CREATED)))
}

fn map_result(result: anyhow::Result<impl Reply>) -> Result<impl Reply, Rejection> {
    result
         .map_err(crate::problem::from_anyhow)
         .map_err(warp::reject::custom)
}
