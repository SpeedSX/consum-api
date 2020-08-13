use serde::{Serialize, Deserialize};
use tiberius::time::chrono::NaiveDateTime;
use crate::serialization::serialize_optional_datetime;

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct Order {
    pub consId: i32,
    pub orderState: i32,
    #[serde(serialize_with = "serialize_optional_datetime")]
    pub incomeDate: Option<NaiveDateTime>,
    pub supplierId: i32,
    pub accountNum: Option<String>,
    #[serde(serialize_with = "serialize_optional_datetime")]
    pub accountDate: Option<NaiveDateTime>,
    pub bySelf: Option<i32>,
    pub hasTrust: bool,
    pub trustSer: Option<String>,
    pub trustNum: Option<i32>,
    pub comment: Option<String>,
    pub enterpriseId: i32
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct Category {
    pub catId: i32,
    pub parentId: Option<i32>,
    pub catName: Option<String>, // is never null in a real DB
    pub catUnitCode: i32,
    pub code: i32
}


#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct Supplier {
    pub supplierId: i32,
    pub supplierName: Option<String>,
    pub supplierPhone: Option<String>,
    pub supplierFax: Option<String>,
    pub supplierManager: Option<String>,
    pub supplierEmail: Option<String>,
    pub supplierAddressDoc: Option<String>,
    pub supplierAddressFact: Option<String>,
    pub supplierAddressStore: Option<String>,
    pub supplierStoreTime: Option<String>,
    pub supplierStoreWho: Option<String>,
    pub supplierStorePhone: Option<String>,
    pub supplierFullName: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateOrder {
    pub accountNum: String,
    pub accountDate: NaiveDateTime,
    pub incomeDate: NaiveDateTime,
    pub hasTrust: bool,
    pub trustSer: Option<String>,
    pub trustNum: Option<i32>,
    pub supplierId: i32,
    pub bySelf: Option<i32>,
    pub comment: String,
    pub enterpriseId: i32
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateSupplier {
    pub supplierName: Option<String>,
    pub supplierPhone: Option<String>,
    pub supplierFax: Option<String>,
    pub supplierManager: Option<String>,
    pub supplierEmail: Option<String>,
    pub supplierAddressDoc: Option<String>,
    pub supplierAddressFact: Option<String>,
    pub supplierAddressStore: Option<String>,
    pub supplierStoreTime: Option<String>,
    pub supplierStoreWho: Option<String>,
    pub supplierStorePhone: Option<String>,
    pub supplierFullName: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateCategory {
    pub parentId: Option<i32>,
    pub catName: String,
    pub catUnitCode: i32,
    pub code: i32
}

#[derive(Debug, Deserialize)]
pub struct ApiKey {
    pub api_key: String,
}

#[derive(Debug)]
pub struct User {
    pub name: String,
}