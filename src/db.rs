use anyhow::Result;
use tiberius::{Row};

use crate::{
    model::*,
    DBPool
};

pub struct DB {
    db_pool: DBPool
}

impl DB {

    pub fn new(db_pool: DBPool) -> DB {
        DB {
            db_pool: db_pool
        }
    }

    pub async fn get_orders(&self) -> Result<Vec<Order>> {

        let mut client = self.db_pool.get().await?;
        
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
                    accountNum: Self::get_string(r, "AccountNum"),
                    accountDate: r.get("AccountDate").into(),
                    bySelf: r.get("BySelf").into(),
                    hasTrust: r.get::<bool, &str>("HasTrust").unwrap_or_default().into(),
                    sellerId: r.get("SellerID").unwrap_or_default(),
                    trustNum: r.get("TrustNum").into(),
                    trustSer: Self::get_string(r, "TrustSer"),
                    comment: Self::get_string(r, "Comment"),
                }})
            .collect();

        info!("Orders count = {}", orders.len());

        Ok(orders)
    }

    pub async fn get_order(&self, id: i32) -> Result<Option<Order>> {

        let mut client = self.db_pool.get().await?;
        
        let stream = client.query("SELECT * from ConsOrders where ConsID = @P1", &[&id]).await?;
        let row = stream.into_row().await?;
        
        let order = row
            .map(|r| {
                //info!("{:?}", r);
                Order { 
                    consId: r.get("ConsID").unwrap_or_default(),
                    orderState: r.get("OrderState").unwrap_or_default(),
                    incomeDate: r.get("IncomeDate").into(),
                    accountNum: Self::get_string(&r, "AccountNum"),
                    accountDate: r.get("AccountDate").into(),
                    bySelf: r.get("BySelf").into(),
                    hasTrust: r.get::<bool, &str>("HasTrust").unwrap_or_default().into(),
                    sellerId: r.get("SellerID").unwrap_or_default(),
                    trustNum: r.get("TrustNum").into(),
                    trustSer: Self::get_string(&r, "TrustSer"),
                    comment: Self::get_string(&r, "Comment"),
                }});

        Ok(order)
    }

    pub async fn create_order(&self, create_order: CreateOrder) -> Result<Option<Order>> {
        let mut client = self. db_pool.get().await?;
        let result = client.query(
                "declare @rc int; exec @rc = up_NewAccount @P1, @P2, @P3, @P4, @P5, @P6, @P7, @P8, @P9; select @rc as Id", 
                &[&create_order.accountNum,
                &create_order.accountDate,
                &create_order.incomeDate,
                &create_order.hasTrust,
                &create_order.trustSer,
                &create_order.trustNum,
                &create_order.sellerId,
                &create_order.bySelf,
                &create_order.comment])
            .await?;
            info!("{:?}", result);
            let result = result
            .into_row()
            .await?;
            
        let id_value: Option<i32> = result.map(|r| r.get("Id")).flatten();
        //id_value.map()
        if let Some(id) = id_value {
            let order = self.get_order(id).await?;
            return Ok(order);
        }
        Ok(None)
        // result.map(|r| Ok(Order {
        //     consId: r.get("Id"),
        //     accountDate: create_order.accountDate,
        //     accountNum: create_order.accountNum,
        //     incomeDate: create_order.incomeDate,
        //     bySelf: create_order.bySelf,
        //     comment: create_order.comment,
        //     hasTrust: create_order.hasTrust,
        //     orderState: 0, // TODO!
        //     sellerId: create_order.sellerId,
        //     trustNum: create_order.trustNum,
        //     trustSer
        // }))
    }

    pub async fn get_categories(&self) -> Result<Vec<Category>> {

        let mut client = self.db_pool.get().await?;
        
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
                    catName: Self::get_string(r, "CatName"),
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
}