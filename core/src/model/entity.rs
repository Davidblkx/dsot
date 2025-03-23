use super::entities::Artist;
use crate::storage::sql::{SqlOperation, SqlOperationHandler};

use crate::error::Result;

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
        trx: sqlx::Transaction<'static, sqlx::Sqlite>,
        op: &SqlOperation,
    ) -> Result<sqlx::Transaction<'static, sqlx::Sqlite>> {
        match Self::from_id(op.get_entity()) {
            Some(DsotEntity::Artist) => Artist::apply_sql_op(trx, op).await,
            _ => todo!(),
        }
    }
}
