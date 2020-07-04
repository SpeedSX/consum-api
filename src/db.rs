use anyhow::Result;
use tiberius::{Row};

use crate::{
    model::Order,
    DBPool
};

pub async fn get_orders(pool: DBPool) -> Result<Vec<Order>> {

    let mut client = pool.get().await.unwrap();
    
    let stream = client.simple_query("SELECT top (100) * from ConsOrders").await?;
    let rows: Vec<Row> = stream.into_first_result().await?;
    
    let orders: Vec<Order> = rows
        .iter()
        .map(|r| {
            Order { 
                consId: r.get("ConsID").unwrap_or(0),
                accountNum: String::from(r.get("AccountNum").unwrap_or("")),
                accountDate: r.get("AccountDate").into()
            }})
        .collect();

    info!(target: "orders", "Orders count = {}", orders.len());

    Ok(orders)
}
