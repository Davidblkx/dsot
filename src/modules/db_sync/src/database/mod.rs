use redb::Database;
use sqlx::SqlitePool;

mod entity_ops;
mod journal;
mod transaction;

pub type DsotDatabaseError = crate::DBSyncError;
pub type Result<T> = std::result::Result<T, crate::DBSyncError>;
pub use transaction::DsotDatabaseTransaction;

#[derive(Debug)]
pub struct DsotDatabase {
    pub(crate) journal: Database,
    pub(crate) sql: SqlitePool,
    pub(crate) id: String,
}

impl DsotDatabase {
    pub fn new(journal: redb::Database, sql: SqlitePool) -> Self {
        Self {
            journal,
            sql,
            id: "db".to_string(),
        }
    }

    pub async fn begin_transaction(&self) -> Result<DsotDatabaseTransaction<'_>> {
        let journal_trx = self.journal.begin_write()?;
        let sql_trx = self.sql.begin().await?;
        Ok(DsotDatabaseTransaction {
            journal_trx,
            sql_trx,
        })
    }

    pub fn with_id<T: Into<String>>(mut self, id: T) -> Self {
        self.id = id.into();
        self
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub async fn close(self) -> Result<()> {
        let DsotDatabase { sql, .. } = self;

        sql.close().await;

        Ok(())
    }
}
