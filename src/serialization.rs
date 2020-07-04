use serde::{Serialize, Serializer};
use tiberius::time::chrono::NaiveDateTime;

pub fn serialize_optional_datetime<S>(d: &Option<NaiveDateTime>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match d {
        Some(v) => (*v).serialize(s),
        None => "".serialize(s)
    }
}
