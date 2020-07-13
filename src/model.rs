use serde::{Serialize, Deserialize};
use tiberius::time::chrono::NaiveDateTime;

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct Order {
    pub consId: i32,
    pub orderState: i32,
    #[serde(serialize_with = "crate::serialization::serialize_optional_datetime")]
    pub incomeDate: Option<NaiveDateTime>,
    pub sellerId: i32,
    pub accountNum: Option<String>,
    #[serde(serialize_with = "crate::serialization::serialize_optional_datetime")]
    pub accountDate: Option<NaiveDateTime>,
    pub bySelf: Option<i32>,
    pub hasTrust: bool,
    pub trustSer: Option<String>,
    pub trustNum: Option<i32>,
    pub comment: Option<String>
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct Category {
    pub catId: i32,
    pub parentId: Option<i32>,
    pub catName: Option<String>,
    pub catUnitCode: i32,
    pub code: i32
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct CreateOrder {
    pub accountNum: String,
    pub accountDate: NaiveDateTime,
    pub incomeDate: NaiveDateTime,
    pub hasTrust: bool,
    pub trustSer: Option<String>,
    pub trustNum: Option<i32>,
    pub sellerId: i32,
    pub bySelf: Option<i32>,
    pub comment: String
}