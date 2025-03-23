use crate::error::Result;

/// Represents a single operation that can be performed on the database.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SQLOperation {
    Create {
        id: uuid::Uuid,
        entity: u32,
        data: Vec<u8>,
    },
    Update {
        id: uuid::Uuid,
        entity: u32,
        action: Vec<u8>
    },
    Delete {
        id: uuid::Uuid,
        entity: u32,
    },
}

impl SQLOperation {
    pub fn get_entity(&self) -> u32 {
        match self {
            SQLOperation::Create { entity, .. } => *entity,
            SQLOperation::Update { entity, .. } => *entity,
            SQLOperation::Delete { entity, .. } => *entity,
        }
    }

    pub fn get_id(&self) -> uuid::Uuid {
        match self {
            SQLOperation::Create { id, .. } => *id,
            SQLOperation::Update { id, .. } => *id,
            SQLOperation::Delete { id, .. } => *id,
        }
    }
}

/// Trait for handling SQL operations.
pub trait SqlOperationHandler {
    fn apply_sql_op(
        trx: sqlx::Transaction<'static, sqlx::Sqlite>,
        op: &SQLOperation,
    ) -> impl Future<Output = Result<sqlx::Transaction<'static, sqlx::Sqlite>>>;
}
