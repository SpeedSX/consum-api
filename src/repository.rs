use anyhow::Result;
use tiberius::{Config, Client, Row};

use super::{
    model::Order,
    configuration::SERVICE_CONFIG,
    connection_manager::{TiberiusConnectionManager}
};
use once_cell::sync::Lazy;

static POOL: Lazy<bb8::Pool<TiberiusConnectionManager>> = Lazy::new(|| {
    // TODO: too many unwraps :)
    let manager = TiberiusConnectionManager::new(Config::from_ado_string(SERVICE_CONFIG.get_connection_string()).unwrap()).unwrap();
    bb8::Pool::builder().max_size(10).build_unchecked(manager)
});

pub async fn get_orders() -> Result<Vec<Order>> {

    let pool = POOL.clone();
    let mut client = pool.get().await.unwrap();

    let stream = client.simple_query("SELECT top (100) * from ConsOrders").await?;
    let rows: Vec<Row> = stream.into_first_result().await?;
    
    let orders: Vec<Order> = rows
        .iter()
        .map(|r| {
            Order { 
                consId: r.get("ConsID").unwrap_or(0),
                accountNum: String::from(r.get("AccountNum").unwrap_or("")),
                accountDate: String::from("")
            }})
        .collect();

    info!(target: "orders", "Orders count = {}", orders.len());

    Ok(orders)
}
