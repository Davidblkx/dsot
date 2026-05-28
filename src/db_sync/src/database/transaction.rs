use super::Result;

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
}
