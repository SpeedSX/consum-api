use anyhow::Result;
use tiberius::{Row};

use crate::{
    model::*,
    DBPool,
    errors::DBRecordNotFound
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
            .map(Self::map_order)
            .collect();

        info!("Orders count = {}", orders.len());

        Ok(orders)
    }

    pub async fn get_order(&self, id: i32) -> Result<Order> {
        let mut client = self.db_pool.get().await?;
        
        let stream = client.query("SELECT * from ConsOrders where ConsID = @P1", &[&id]).await?;
        let row = stream.into_row().await?;

        if let Some(order_row) = row {
            let order = Self::map_order(&order_row);
            return Ok(order);
        }

        anyhow::bail!(DBRecordNotFound)
    }

    pub async fn create_order(&self, create_order: CreateOrder) -> Result<Order> {
        let mut client = self. db_pool.get().await?;
        let result = client.query(
                "set nocount on; declare @rc int; exec @rc = up_NewAccount @P1, @P2, @P3, @P4, @P5, @P6, @P7, @P8, @P9; select @rc as Id", 
                &[&create_order.accountNum,
                &create_order.accountDate,
                &create_order.incomeDate,
                &create_order.hasTrust,
                &create_order.trustSer,
                &create_order.trustNum,
                &create_order.sellerId,
                &create_order.bySelf,
                &create_order.comment])
            .await?
            .into_row()
            .await?;

        if let Some(row) = result {
            let id_value: Option<i32> = row.try_get("Id").ok().flatten();
            if let Some(id) = id_value {
                let order = self.get_order(id).await?;
                return Ok(order);
            }
        }

        anyhow::bail!(DBRecordNotFound)
    }

    pub async fn get_category(&self, id: i32) -> Result<Option<Category>> {
        let mut client = self.db_pool.get().await?;
        
        let stream = client.query("SELECT * from ConsCats where CatID = @P1", &[&id]).await?;
        let row = stream.into_row().await?;
        
        let category = row.as_ref().map(Self::map_category);

        Ok(category)
    }

    pub async fn get_categories(&self) -> Result<Vec<Category>> {

        let mut client = self.db_pool.get().await?;
        
        let stream = client.simple_query("SELECT * from ConsCats").await?;
        let rows: Vec<Row> = stream.into_first_result().await?;
        
        let cats: Vec<Category> = rows
            .iter()
            .map(Self::map_category)
            .collect();

        info!("Cats count = {}", cats.len());

        Ok(cats)
    }

    fn map_order(row: &Row) -> Order {
        trace!("Mapping row to order: {:?}", row);
        Order { 
            consId: row.get("ConsID").unwrap_or_default(),
            orderState: row.get("OrderState").unwrap_or_default(),
            incomeDate: row.get("IncomeDate").into(),
            accountNum: Self::get_string(row, "AccountNum"),
            accountDate: row.get("AccountDate").into(),
            bySelf: row.get("BySelf").into(),
            hasTrust: row.get::<bool, &str>("HasTrust").unwrap_or_default().into(),
            sellerId: row.get("SellerID").unwrap_or_default(),
            trustNum: row.get("TrustNum").into(),
            trustSer: Self::get_string(row, "TrustSer"),
            comment: Self::get_string(row, "Comment"),
        }
    }

    fn map_category(row: &Row) -> Category {
        trace!("Mapping row to category: {:?}", row);
        Category { 
            catId: row.get("CatID").unwrap_or_default(),
            // TODO: only try_get-unwrap_or works for this field, while for ConsOrders.TrustNum we can use just .into() (see above)
            // Possibly because it comes as I8(None) instead of i32, but why?
            parentId: row.try_get("ParentID").unwrap_or_default(),
            catName: Self::get_string(row, "CatName"),
            catUnitCode: row.get("CatUnitCode").unwrap_or_default(),
            code: row.get("Code").unwrap_or_default(),
        }
    }

    fn get_string(r: &Row, col: &str) -> Option<String> {
        r.get::<&str, &str>(col).map(|s| s.to_string())
    }
}