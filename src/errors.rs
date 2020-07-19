#[derive(thiserror::Error, Debug, Clone, Copy)]
#[error("Record not found in database")]
pub struct DBRecordNotFound;