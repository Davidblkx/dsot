use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod update_value;

pub use update_value::{IntoUpdateValue, UpdateValue};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct JournalEntry {
    pub id: Uuid,
    pub table: String,
    pub op: SyncOperation,
}

impl JournalEntry {
    pub fn new(table: &str, op: &SyncOperation) -> Self {
        Self {
            id: Uuid::now_v7(),
            table: table.to_string(),
            op: op.clone(),
        }
    }

    pub fn create_entry(table: &str, op: &SyncOperation) -> crate::dser::Result<(Uuid, Vec<u8>)> {
        let jrn = Self::new(table, op);
        Ok((jrn.id, jrn.to_bytes()?))
    }

    pub fn to_bytes(self) -> crate::dser::Result<Vec<u8>> {
        crate::dser::EntityMessagePack::serialize(self)
    }

    pub fn from_bytes(data: &[u8]) -> crate::dser::Result<Self> {
        crate::dser::EntityMessagePack::deserialize(data)
    }
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
