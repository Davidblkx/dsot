use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod update_value;

pub use update_value::{IntoUpdateValue, UpdateValue};

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
