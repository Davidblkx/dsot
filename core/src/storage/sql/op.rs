use super::{SqlResult, SqlTransaction};

/// Represents a single operation that can be performed on the database.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SqlOperation {
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

impl SqlOperation {
    pub fn get_entity(&self) -> u32 {
        match self {
            SqlOperation::Create { entity, .. } => *entity,
            SqlOperation::Update { entity, .. } => *entity,
            SqlOperation::Delete { entity, .. } => *entity,
        }
    }

    pub fn get_id(&self) -> uuid::Uuid {
        match self {
            SqlOperation::Create { id, .. } => *id,
            SqlOperation::Update { id, .. } => *id,
            SqlOperation::Delete { id, .. } => *id,
        }
    }
}

/// Trait for handling SQL operations.
pub trait SqlOperationHandler {
    fn apply_sql_op(
        trx: SqlTransaction,
        op: &SqlOperation,
    ) -> impl Future<Output = SqlResult<()>>;
}
