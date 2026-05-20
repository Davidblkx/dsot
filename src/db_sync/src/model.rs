use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SyncRow {
    pub id: Uuid,
    pub table: String,
    pub op: SyncOperation,
    pub date: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum SyncOperation {
    Delete(Uuid),
    Restore(Uuid),
    Update(Uuid, Vec<UpdateColumnOp>),
    Create(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UpdateColumnOp {
    pub column: String,
    pub value: UpdateValue,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "t", content = "v")]
pub enum UpdateValue {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
}

impl UpdateValue {
    pub fn get_if_diff<T, U>(v1: &T, v2: &U) -> Option<UpdateValue>
    where
        T: PartialEq<U> + ?Sized,
        U: ?Sized,
        for<'a> &'a U: Into<UpdateValue>,
    {
        if v1 != v2 { Some(v2.into()) } else { None }
    }
}

impl From<String> for UpdateValue {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<&str> for UpdateValue {
    fn from(value: &str) -> Self {
        Self::Text(value.to_string())
    }
}

impl From<i64> for UpdateValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<f64> for UpdateValue {
    fn from(value: f64) -> Self {
        Self::Real(value)
    }
}

impl From<bool> for UpdateValue {
    fn from(value: bool) -> Self {
        Self::Integer(if value { 1 } else { 0 })
    }
}

impl From<Option<String>> for UpdateValue {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(v) => v.into(),
            None => Self::Null,
        }
    }
}
