use redb::Database;
use sqlx::SqlitePool;

mod entity_ops;
mod error;
mod journal;
mod transaction;

pub use error::{DsotDatabaseError, Result};
pub use transaction::DsotDatabaseTransaction;

#[derive(Debug)]
pub struct DsotDatabase {
    pub(crate) journal: Database,
    pub(crate) sql: SqlitePool,
}

impl DsotDatabase {
    pub fn new(journal: redb::Database, sql: SqlitePool) -> Self {
        Self { journal, sql }
    }

    pub async fn begin_transaction(&self) -> Result<DsotDatabaseTransaction<'_>> {
        let journal_trx = self.journal.begin_write()?;
        let sql_trx = self.sql.begin().await?;
        Ok(DsotDatabaseTransaction {
            journal_trx,
            sql_trx,
        })
    }
}
