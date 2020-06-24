use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Order {
    pub consId: i32,
    pub accountNum: String,
    pub accountDate: String
}
