use redb::ReadableTable;

use super::Result;
use super::journal::JOURNAL_TABLE;

pub struct DsotDatabaseTransaction<'a> {
    pub(crate) journal_trx: redb::WriteTransaction,
    pub(crate) sql_trx: sqlx::Transaction<'a, sqlx::Sqlite>,
}

impl<'a> DsotDatabaseTransaction<'a> {
    pub async fn commit(self) -> Result<()> {
        log::trace!("Committing database transaction");
        self.journal_trx.commit()?;
        self.sql_trx.commit().await?;
        Ok(())
    }

    pub async fn rollback(self) -> Result<()> {
        log::debug!("Rolling back database transaction");
        self.journal_trx.abort()?;
        self.sql_trx.rollback().await?;
        Ok(())
    }

    pub fn generate_sync_hash(&self) -> Result<[u8; 32]> {
        let table = self.journal_trx.open_table(JOURNAL_TABLE)?;
        let mut hasher = blake3::Hasher::new();
        let range = table.range::<[u8; 16]>(..)?;

        for result in range.into_iter() {
            let (key, _value) = result?;
            hasher.update(key.value().as_slice());
        }
        let finalized = hasher.finalize();

        Ok(finalized.into())
    }
}
