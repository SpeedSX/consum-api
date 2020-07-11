use anyhow::Result;
use tiberius::{Row};

use crate::{
    model::{Category, Order},
    DBPool
};

pub async fn get_orders(pool: DBPool) -> Result<Vec<Order>> {

    let mut client = pool.get().await?;
    
    let stream = client.simple_query("SELECT top (100) * from ConsOrders").await?;
    let rows: Vec<Row> = stream.into_first_result().await?;
    
    let orders: Vec<Order> = rows
        .iter()
        .map(|r| {
            //info!("{:?}", r);
            Order { 
                consId: r.get("ConsID").unwrap_or_default(),
                orderState: r.get("OrderState").unwrap_or_default(),
                incomeDate: r.get("IncomeDate").into(),
                accountNum: get_string(r, "AccountNum"),
                accountDate: r.get("AccountDate").into(),
                bySelf: r.get("BySelf").into(),
                hasTrust: r.get::<bool, &str>("HasTrust").unwrap_or_default().into(),
                sellerId: r.get("SellerID").unwrap_or_default(),
                trustNum: r.get("TrustNum").into(),
                trustSer: get_string(r, "TrustSer"),
                comment: get_string(r, "Comment"),
            }})
        .collect();

    info!("Orders count = {}", orders.len());

    Ok(orders)
}


pub async fn get_categories(pool: DBPool) -> Result<Vec<Category>> {

    let mut client = pool.get().await?;
    
    let stream = client.simple_query("SELECT * from ConsCats").await?;
    let rows: Vec<Row> = stream.into_first_result().await?;
    
    let cats: Vec<Category> = rows
        .iter()
        .map(|r| {
            //debug!("{:?}", r);
            Category { 
                catId: r.get("CatID").unwrap_or_default(),
                // TODO: only try_get-unwrap_or works for this field, while for ConsOrders.TrustNum we can use just .into() (see above)
                // Possibly because it comes as I8(None) instead of i32, but why?
                parentId: r.try_get("ParentID").unwrap_or_default(),
                catName: get_string(r, "CatName"),
                catUnitCode: r.get("CatUnitCode").unwrap_or_default(),
                code: r.get("Code").unwrap_or_default(),
            }})
        .collect();

    info!("Cats count = {}", cats.len());

    Ok(cats)
}

fn get_string(r: &Row, col: &str) -> Option<String> {
    r.get::<&str, &str>(col).map(|s| s.to_string())
}