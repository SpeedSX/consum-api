use anyhow::{bail, Result, Context};
use tiberius::{Row, FromSql};

use crate::{
    model::*,
    DBPool,
    errors::{MissingRequiredField, DBRecordNotFound}
};

trait RowExt {
    // fn get_string(&self, col: &str) -> Option<String>;
    // fn get_value<'a, T>(&'a self, col: &str) -> T where T: Default + FromSql<'a>;
    // fn get_optional<'a, T>(&'a self, col: &str) -> Option<T> where T: FromSql<'a>;

    fn try_get_string(&self, col: &str) -> Result<Option<String>>;
    fn try_get_value<'a, T>(&'a self, col: &str) -> Result<T> where T: Default + FromSql<'a>;
    fn try_get_optional<'a, T>(&'a self, col: &str) -> Result<Option<T>> where T: FromSql<'a>;
    fn try_get_required<'a, T>(&'a self, col: &str) -> Result<T> where T: FromSql<'a>;
}

impl RowExt for Row {
    // methods which ignore errors - not used now 

    // fn get_string(&self, col: &str) -> Option<String> {
    //     self.try_get::<&str, &str>(col).ok().flatten().map(|s| s.to_string())
    // }

    // fn get_value<'a, T>(&'a self, col: &str) -> T where T: Default + FromSql<'a> {
    //     self.try_get::<'a, T, &str>(col).ok().flatten().unwrap_or_default()
    // }

    // fn get_optional<'a, T>(&'a self, col: &str) -> Option<T> where T: FromSql<'a> {
    //     self.try_get::<'a, T, &str>(col).ok().unwrap_or_default()
    // }

    fn try_get_string(&self, col: &str) -> Result<Option<String>> {
        let value = self.try_get::<&str, &str>(col)?;
        Ok(value.map(|s| s.to_string()))
    }

    fn try_get_value<'a, T>(&'a self, col: &str) -> Result<T> where T: Default + FromSql<'a> {
        let value = self.try_get::<'a, T, &str>(col).with_context(|| format!("Failed to retrieve value from field '{}'", col))?;
        Ok(value.unwrap_or_default())
    }

    fn try_get_optional<'a, T>(&'a self, col: &str) -> Result<Option<T>> where T: FromSql<'a> {
         let value = self.try_get::<'a, T, &str>(col).with_context(|| format!("Failed to retrieve optional value from field '{}'", col))?;
         Ok(value)
    }

    fn try_get_required<'a, T>(&'a self, col: &str) -> Result<T> where T: FromSql<'a> {
        let value = self.try_get::<'a, T, &str>(col).with_context(|| format!("Failed to retrieve required value from field '{}'", col))?;
        if let Some(v) = value {
            return Ok(v);
        }

        bail!(MissingRequiredField)
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
        
        let orders: Result<Vec<_>> = rows
            .iter()
            .map(Self::try_map_order)
            .collect();

        if let Ok(list) = orders {
            info!("Orders count = {}", list.len());
            return Ok(list);
        }

        orders
    }

    pub async fn get_orders_filtered(&self, filter: ViewFilter) -> Result<Vec<OrderView>> {
        let mut client = self.db_pool.get().await?;

        let mut query_sql = "select ConsID, EnterpriseID, IncomeDate, AccountNum, AccountDate, \
           ISNULL((select sum(AccountGrn) from ConsOrderItem coi where coi.ConsID = cr.ConsID), 0) as AccountGrn, \
           cr.SellerID, BySelf, HasTrust, TrustSer, TrustNum, \
           ISNULL((select sum(PaidGrn) from ConsPayment cp where cp.ConsID = cr.ConsID), 0) as PaidGrn, cr.Comment \
           from ConsOrders cr left join Seller s on cr.SellerID = s.SellerID".to_string();
        if let Some(order) = filter.orderBy {
            query_sql.push_str(&(" order by ".to_string() + &order));
        }
        let stream = client.simple_query(query_sql).await?;
        let rows: Vec<Row> = stream.into_first_result().await?;
        
        let orders: Result<Vec<_>> = rows
            .iter()
            .map(Self::try_map_order_view)
            .collect();

        if let Ok(list) = orders {
            info!("Orders count = {}", list.len());
            return Ok(list);
        }

        orders
    }

    pub async fn get_order(&self, id: i32) -> Result<Order> {
        let mut client = self.db_pool.get().await?;
        
        let stream = client.query("SELECT * from ConsOrders where ConsID = @P1", &[&id]).await?;
        let row = stream.into_row().await?;

        if let Some(order_row) = row {
            let order = Self::try_map_order(&order_row)?;
            return Ok(order);
        }

        bail!(DBRecordNotFound)
    }

    pub async fn create_order(&self, create_order: CreateOrder) -> Result<Order> {
        let mut client = self.db_pool.get().await?;
        let result = client.query(
                "declare @rc int; exec @rc = up_NewAccount @P1, @P2, @P3, @P4, @P5, @P6, @P7, @P8, @P9, @P10; select @rc as Id", 
                &[&create_order.accountNum,
                &create_order.accountDate,
                &create_order.incomeDate,
                &create_order.hasTrust,
                &create_order.trustSer,
                &create_order.trustNum,
                &create_order.supplierId,
                &create_order.bySelf,
                &create_order.comment,
                &create_order.enterpriseId])
            .await?
            .into_row()
            .await?;

        if let Some(row) = result {
            let id_value = row.try_get::<i32, &str>("Id")?;
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
            let cat = Self::try_map_category(&cat_row)?;
            return Ok(cat);
        }

        bail!(DBRecordNotFound)
    }

    pub async fn get_categories(&self) -> Result<Vec<Category>> {
        let mut client = self.db_pool.get().await?;
        
        let stream = client.simple_query("SELECT * from ConsCats").await?;
        let rows: Vec<Row> = stream.into_first_result().await?;
        
        let mapped_cats: Result<Vec<_>> = rows
            .iter()
            .map(Self::try_map_category)
            .collect();

        if let Ok(cats) = mapped_cats {
            info!("Cats count = {}", cats.len());
            return Ok(cats);
        }

        mapped_cats
    }

    pub async fn create_category(&self, create_cat: CreateCategory) -> Result<Category> {
        let mut client = self.db_pool.get().await?;
        let result = client.query(
                "insert into ConsCats (ParentID, CatName, CatUnitCode, Code) values (@P1, @P2, @P3, @P4); select CAST(SCOPE_IDENTITY() as int) as Id", 
                &[&create_cat.parentId,
                &create_cat.catName,
                &create_cat.catUnitCode,
                &create_cat.code])
            .await?
            .into_row()
            .await?;

        if let Some(row) = result {
            let id_value = row.try_get::<i32, &str>("Id")?;
            if let Some(id) = id_value {
                let cat = self.get_category(id).await?;
                return Ok(cat);
            }
        }

        bail!(DBRecordNotFound)
    }

    pub async fn delete_category(&self, id: i32) -> Result<()> {
        let mut client = self.db_pool.get().await?;
        
        let result = client.execute("DELETE from ConsCats where CatID = @P1", &[&id]).await?;

        if let Some(count) = result.rows_affected().first() {
            if count > &0 {
                return Ok(())
            }
        }

        bail!(DBRecordNotFound)
    }

    pub async fn get_supplier_by_id(&self, id: i32) -> Result<Supplier> {
        let mut client = self.db_pool.get().await?;
        
        let stream = client.query("SELECT * from Seller where SellerID = @P1", &[&id]).await?;
        let row = stream.into_row().await?;

        if let Some(seller_row) = row {
            let seller = Self::try_map_supplier(&seller_row)?;
            return Ok(seller);
        }

        bail!(DBRecordNotFound)
    }

    pub async fn get_supplier_by_name(&self, name: String) -> Result<Supplier> {
        let mut client = self.db_pool.get().await?;

        let stream = client.query("SELECT * from Seller where SellerName = @P1", &[&name]).await?;
        let row = stream.into_row().await?;

        if let Some(seller_row) = row {
            let seller = Self::try_map_supplier(&seller_row)?;
            return Ok(seller);
        }

        bail!(DBRecordNotFound)
    }

    pub async fn create_supplier(&self, create_supplier: CreateSupplier) -> Result<Supplier> {
        let mut client = self.db_pool.get().await?;
        let result = client.query(
                "insert into Seller (SellerName, SellerPhone, SellerFax, SellerManager, SellerEmail, SellerAddressDoc, SellerAddressFact, SellerAddressStore, SellerStoreTime, SellerStoreWho, SellerStorePhone, SellerFullName) \
                values (@P1, @P2, @P3, @P4, @P5, @P6, @P7, @P8, @P9, @P10, @P11, @P12); select CAST(SCOPE_IDENTITY() as int) as Id", 
                &[&create_supplier.supplierName,
                &create_supplier.supplierPhone,
                &create_supplier.supplierFax,
                &create_supplier.supplierManager,
                &create_supplier.supplierEmail,
                &create_supplier.supplierAddressDoc,
                &create_supplier.supplierAddressFact,
                &create_supplier.supplierAddressStore,
                &create_supplier.supplierStoreTime,
                &create_supplier.supplierStoreWho,
                &create_supplier.supplierStorePhone,
                &create_supplier.supplierFullName
                ])
            .await?
            .into_row()
            .await?;

        if let Some(row) = result {
            //debug!("{:?}", r);
            let id_value = row.try_get::<i32, &str>("Id")?;
            if let Some(id) = id_value {
                let supplier = self.get_supplier_by_id(id).await?;
                return Ok(supplier);
            }
        }

        bail!(DBRecordNotFound)
    }

    // select PayID, cr.SellerID, PayDate, PaidGrn, cp.ConsID, AccountNum, PayDocNum
    // from ConsPayment cp
    //   inner join ConsOrders cr on cp.ConsID = cr.ConsID
    //   left join Seller s on cr.SellerID = s.SellerID
    // order by PayDate
    
    // select ItemID, ConsID, Num, CatCode, AccountGrn, AccountPrice, ManualFix
    // from ConsOrderItem
    // where ConsID = @P1
    // order by ItemID

    // select PayID, ConsID, PayDate, PaidGrn, PayDocNum
    // from ConsPayment
    // where ConsID = @P1
    // order by PayDate

    // select ReqID, RequestState, RequestDate, UserCode, CatCode, NeedDate, Num, CancelRequest, RefuseRequest
    // from ConsReqs
    // where CancelRequest = 0 and RefuseRequest = 0
    // order by RequestDate

    fn try_map_order(row: &Row) -> Result<Order> {
        trace!("Try mapping row to order: {:?}", row);
        Ok(Order { 
            consId: row.try_get_required("ConsID")?,
            orderState: row.try_get_value("OrderState")?,
            incomeDate: row.try_get_optional("IncomeDate")?,
            accountNum: row.try_get_string("AccountNum")?,
            accountDate: row.try_get_optional("AccountDate")?,
            bySelf: row.try_get_optional("BySelf")?,
            hasTrust: row.try_get_value("HasTrust")?,
            supplierId: row.try_get_value("SellerID")?,
            trustNum: row.try_get_optional("TrustNum")?,
            trustSer: row.try_get_string("TrustSer")?,
            comment: row.try_get_string("Comment")?,
            enterpriseId: row.try_get_value("EnterpriseID")?,
        })
    }

    fn try_map_order_view(row: &Row) -> Result<OrderView> {
        trace!("Try mapping row to order view: {:?}", row);
        Ok(OrderView { 
            consId: row.try_get_required("ConsID")?,
            incomeDate: row.try_get_optional("IncomeDate")?,
            accountNum: row.try_get_string("AccountNum")?,
            accountDate: row.try_get_optional("AccountDate")?,
            bySelf: row.try_get_optional("BySelf")?,
            hasTrust: row.try_get_value("HasTrust")?,
            supplierId: row.try_get_value("SellerID")?,
            trustNum: row.try_get_optional("TrustNum")?,
            trustSer: row.try_get_string("TrustSer")?,
            comment: row.try_get_string("Comment")?,
            enterpriseId: row.try_get_value("EnterpriseID")?,
            paidGrn: row.try_get_value("PaidGrn")?,
            accountGrn: row.try_get_value("AccountGrn")?,
        })
    }

    fn try_map_category(row: &Row) -> Result<Category> {
        trace!("Try mapping row to category: {:?}", row);
        Ok(Category { 
            catId: row.try_get_required("CatID")?,
            parentId: row.try_get_optional("ParentID")?,
            catName: row.try_get_string("CatName")?,
            catUnitCode: row.try_get_value("CatUnitCode")?,
            code: row.try_get_value("Code")?,
        })
    }

    fn try_map_supplier(row: &Row) -> Result<Supplier> {
        trace!("Try mapping row to supplier: {:?}", row);
        Ok(Supplier { 
            supplierId: row.try_get_required("SellerID")?,
            supplierName: row.try_get_string("SellerName")?,
            supplierPhone: row.try_get_string("SellerPhone")?,
            supplierFax: row.try_get_string("SellerFax")?,
            supplierManager: row.try_get_string("SellerManager")?,
            supplierEmail: row.try_get_string("SellerEmail")?,
            supplierAddressDoc: row.try_get_string("SellerAddressDoc")?,
            supplierAddressFact: row.try_get_string("SellerAddressFact")?,
            supplierAddressStore: row.try_get_string("SellerAddressStore")?,
            supplierStoreTime: row.try_get_string("SellerStoreTime")?,
            supplierStoreWho: row.try_get_string("SellerStoreWho")?,
            supplierStorePhone: row.try_get_string("SellerStorePhone")?,
            supplierFullName: row.try_get_string("SellerFullName")?,
        })
    }
}