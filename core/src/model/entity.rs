use super::entities::ArtistSql;
use crate::storage::sql::{SqlOperation, SqlOperationHandler, SqlResult, SqlTransaction};

pub enum DsotEntity {
    Artist,
    ArtistAliases,
}

impl DsotEntity {
    pub fn get_id(&self) -> u32 {
        match self {
            DsotEntity::Artist => 1,
            DsotEntity::ArtistAliases => 2,
        }
    }

    pub fn from_id(id: u32) -> Option<Self> {
        match id {
            1 => Some(DsotEntity::Artist),
            2 => Some(DsotEntity::ArtistAliases),
            _ => None,
        }
    }
}

impl SqlOperationHandler for DsotEntity {
    async fn apply_sql_op(
        trx: SqlTransaction,
        op: &SqlOperation,
    ) -> SqlResult<()> {
        match Self::from_id(op.get_entity()) {
            Some(DsotEntity::Artist) => ArtistSql::execute_operation(trx, op).await,
            _ => todo!(),
        }
    }
}
