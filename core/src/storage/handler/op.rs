use core::result::Result::Ok;

use super::{SqliteDbHandler, error::DatabaseHandlerError};
use crate::error::Result;
use crate::model::{DsotEntity, JournalEntry, JournalOperation};
use crate::storage::redb::RedbTransaction;
use crate::storage::{
    BinModel, SqlOperation, SqlOperationHandler, SqlTransaction, StorageEntity, StorageTransaction,
};

impl SqliteDbHandler {
    /// Check if a journal entry was applied
    pub fn has(&self, entry: &JournalEntry) -> Result<bool> {
        let has_key = self
            .create_journal_transaction()?
            .get(&entry.get_storage_key())?
            .is_some();

        Ok(has_key)
    }

    /// Add a new entry to the journal and applies it to the database
    ///
    /// If entry was already added, an error is returned
    pub async fn apply(&self, entry: &JournalEntry) -> Result<()> {
        if self.has(entry)? {
            return DatabaseHandlerError::DuplicatedEntry.to_err();
        }

        let mut journal_trx: RedbTransaction = self.create_journal_transaction()?;

        journal_trx.set(&entry.get_storage_key(), &entry.serialize()?)?;

        match &entry.op {
            JournalOperation::SQL(op) => self.apply_sql_operation(journal_trx, op).await,
        }
    }

    /// Creates a new entry for an operation and applies it to the database
    pub async fn create_and_apply(&self, op: JournalOperation) -> Result<JournalEntry> {
        let entry = JournalEntry::new(op);
        self.apply(&entry).await?;
        Ok(entry)
    }

    async fn apply_sql_operation(&self, pending: RedbTransaction, op: &SqlOperation) -> Result<()> {
        let trx: SqlTransaction = self.create_db_transaction().await?;

        let (trx, _): (SqlTransaction, _) = DsotEntity::apply_sql_op(trx, op).await?;
        trx.commit().await?;
        pending.commit()?;

        Ok(())
    }
}
