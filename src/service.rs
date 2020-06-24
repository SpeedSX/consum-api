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
            Order { 
                consId: r.get("ConsID").unwrap_or(0),
                accountNum: r.get("AccountNum").unwrap_or("").to_owned(),
                accountDate: "".to_string()
            }})
        .collect();

    info!(target: "orders", "Orders count = {}", orders.len());

    Ok(orders)
}
