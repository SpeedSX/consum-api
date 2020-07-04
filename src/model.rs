use serde::{Serialize};
use tiberius::time::chrono::NaiveDateTime;

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct Order {
    pub consId: i32,
    pub accountNum: String,
    #[serde(serialize_with = "crate::serialization::serialize_optional_datetime")]
    pub accountDate: Option<NaiveDateTime>
}
