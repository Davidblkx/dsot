use redb::{Database, TableDefinition};
use sqlx::SqlitePool;
use thiserror::Error;
use uuid::Uuid;

use crate::entity::SyncEntity;
use crate::model::{JournalEntry, SyncOperation};
use crate::repo::SyncEntityRepository;

const JOURNAL_TABLE: TableDefinition<[u8; 16], &[u8]> = TableDefinition::new("JOURNAL");

#[derive(Debug, Error)]
pub enum DsotDatabaseError {
    #[error("Redb storage failure: {0}")]
    RedbStorage(#[from] redb::StorageError),
    #[error("Redb transaction failed or aborted: {0}")]
    RedbTransaction(#[from] redb::TransactionError),
    #[error("Redb commit failed: {0}")]
    RedbCommit(#[from] redb::CommitError),
    #[error("Redb table error: {0}")]
    RedbTable(#[from] redb::TableError),
    #[error("Repository error: {0}")]
    RepositoryError(#[from] crate::repo::RepositoryError),
    #[error("SQLite error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("{0}")]
    SerializationError(#[from] crate::dser::MessagePackError),
}

pub type Result<T> = std::result::Result<T, DsotDatabaseError>;

#[derive(Debug)]
pub struct DsotDatabase {
    journal: Database,
    sql: SqlitePool,
}

impl DsotDatabase {
    pub fn new(journal: redb::Database, sql: SqlitePool) -> Self {
        Self { journal, sql }
    }

    pub async fn get<R: SyncEntityRepository>(&self, id: uuid::Uuid) -> Result<R::RepoEntity> {
        let row = R::get(&self.sql, id).await?;
        Ok(row)
    }

    pub async fn insert<R: SyncEntityRepository>(&self, value: &R::RepoEntity) -> Result<()> {
        let op = value.op_create()?;
        self.exec_op::<R>(op).await?;
        Ok(())
    }

    async fn exec_op<R: SyncEntityRepository>(&self, op: SyncOperation) -> Result<Uuid> {
        let (jrn_id, jrn_bytes) = JournalEntry::create_entry(R::get_table_name(), &op)?;
        let jrn_trx = self.journal.begin_write()?;
        {
            let mut table = jrn_trx.open_table(JOURNAL_TABLE)?;
            table.insert(jrn_id.to_bytes_le(), jrn_bytes.as_slice())?;
        }

        let mut sql_trx = self.sql.begin().await?;
        R::exec_op(&mut *sql_trx, op).await?;

        jrn_trx.commit()?;
        match sql_trx.commit().await {
            Ok(_) => Ok(jrn_id),
            Err(e) => {
                self.remove_journal_entry(jrn_id)?;
                Err(DsotDatabaseError::DatabaseError(e))
            }
        }
    }

    fn remove_journal_entry(&self, id: Uuid) -> Result<()> {
        let jrn_trx = self.journal.begin_write()?;
        {
            let mut table = jrn_trx.open_table(JOURNAL_TABLE)?;
            table.remove(&id.to_bytes_le())?;
        }
        jrn_trx.commit()?;

        Ok(())
    }
}
