#[derive(thiserror::Error, Debug, Clone, Copy)]
#[error("Record not found in database")]
pub struct DBRecordNotFound;

#[derive(thiserror::Error, Debug, Clone, Copy)]
#[error("Required field value not found")]
pub struct MissingRequiredField;
