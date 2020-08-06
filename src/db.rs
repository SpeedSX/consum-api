use anyhow::{bail, Result};
use tiberius::{Row, FromSql};

use crate::{
    model::*,
    DBPool,
    errors::DBRecordNotFound
};

trait RowExt {
    fn get_string(&self, col: &str) -> Option<String>;
    fn get_value<'a, T>(&'a self, col: &str) -> T where T: Default + FromSql<'a>;
    fn get_optional<'a, T>(&'a self, col: &str) -> Option<T> where T: FromSql<'a>;
 }

impl RowExt for Row {
    // TODO: for now we ignore errors here, and just return 'None' in case of incorrect column name or type

    fn get_string(&self, col: &str) -> Option<String> {
        self.try_get::<&str, &str>(col).ok().flatten().map(|s| s.to_string())
    }

    fn get_value<'a, T>(&'a self, col: &str) -> T where T: Default + FromSql<'a> {
        self.try_get::<'a, T, &str>(col).ok().flatten().unwrap_or_default()
    }

    fn get_optional<'a, T>(&'a self, col: &str) -> Option<T> where T: FromSql<'a> {
        self.try_get::<'a, T, &str>(col).ok().unwrap_or_default()
    }
}

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

        bail!(DBRecordNotFound)
    }

    pub async fn create_order(&self, create_order: CreateOrder) -> Result<Order> {
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

        bail!(DBRecordNotFound)
    }

    pub async fn get_category(&self, id: i32) -> Result<Category> {
        let mut client = self.db_pool.get().await?;
        
        let stream = client.query("SELECT * from ConsCats where CatID = @P1", &[&id]).await?;
        let row = stream.into_row().await?;
        
        if let Some(cat_row) = row {
            let cat = Self::map_category(&cat_row);
            return Ok(cat);
        }

        bail!(DBRecordNotFound)
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

    pub async fn create_category(&self, create_cat: CreateCategory) -> Result<Category> {
        let mut client = self. db_pool.get().await?;
        let result = client.query(
                "insert into ConsCats (ParentID, CatName, CatUnitCode, Code) values (@P1, @P2, @P3, @P4); select SCOPE_IDENTITY() as Id", 
                &[&create_cat.parentId,
                &create_cat.catName,
                &create_cat.catUnitCode,
                &create_cat.code])
            .await?
            .into_row()
            .await?;

        if let Some(row) = result {
            let id_value: Option<i32> = row.try_get("Id").ok().flatten();
            if let Some(id) = id_value {
                let cat = self.get_category(id).await?;
                return Ok(cat);
            }
        }

        bail!(DBRecordNotFound)
    }

    fn map_order(row: &Row) -> Order {
        trace!("Mapping row to order: {:?}", row);
        Order { 
            consId: row.get_value("ConsID"),
            orderState: row.get_value("OrderState"),
            incomeDate: row.get_optional("IncomeDate"),
            accountNum: row.get_string("AccountNum"),
            accountDate: row.get_optional("AccountDate"),
            bySelf: row.get_optional("BySelf"),
            hasTrust: row.get_value("HasTrust"),
            sellerId: row.get_value("SellerID"),
            trustNum: row.get_optional("TrustNum"),
            trustSer: row.get_string("TrustSer"),
            comment: row.get_string("Comment"),
        }
    }

    fn map_category(row: &Row) -> Category {
        trace!("Mapping row to category: {:?}", row);
        Category { 
            catId: row.get_value("CatID"),
            parentId: row.get_optional("ParentID"),
            catName: row.get_string("CatName"),
            catUnitCode: row.get_value("CatUnitCode"),
            code: row.get_value("Code"),
        }
    }
}