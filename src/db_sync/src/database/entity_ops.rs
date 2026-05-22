use uuid::Uuid;

use crate::entity::SyncEntity;
use crate::model::{JournalEntry, SyncOperation};
use crate::repo::{ListQuery, SyncEntityRepository};

use super::{DsotDatabase, DsotDatabaseError, Result, journal::JOURNAL_TABLE};

impl DsotDatabase {
    /// Tries to get the entity with the given ID. Returns None if entity does not exist, or an error if retrieval fails.
    pub async fn try_get<R: SyncEntityRepository>(
        &self,
        id: uuid::Uuid,
    ) -> Result<Option<R::RepoEntity>> {
        let mut conn = self
            .sql
            .acquire()
            .await
            .map_err(DsotDatabaseError::DatabaseError)?;
        let row = R::try_get(&mut *conn, id).await?;
        Ok(row)
    }

    /// Gets the entity with the given ID. Returns an error if entity does not exist or retrieval fails.
    pub async fn get<R: SyncEntityRepository>(&self, id: uuid::Uuid) -> Result<R::RepoEntity> {
        let mut conn = self
            .sql
            .acquire()
            .await
            .map_err(DsotDatabaseError::DatabaseError)?;
        let row = R::get(&mut *conn, id).await?;
        Ok(row)
    }

    /// Inserts the given entity. Returns an error if entity already exists or if creation fails.
    pub async fn insert<R: SyncEntityRepository>(&self, value: &R::RepoEntity) -> Result<()> {
        let op = value.op_create()?;
        self.exec_op::<R>(op).await?;
        Ok(())
    }

    /// Updates the given entity if it exists and has changes. Returns false if entity does not exist or no update is needed.
    pub async fn update<R: SyncEntityRepository>(&self, value: &R::RepoEntity) -> Result<bool> {
        let mut conn = self
            .sql
            .acquire()
            .await
            .map_err(DsotDatabaseError::DatabaseError)?;
        match R::try_get(&mut *conn, value.get_id()).await? {
            Some(prev) => match value.op_update(&prev) {
                Some(op) => {
                    self.exec_op::<R>(op).await?;
                    Ok(true)
                }
                None => Ok(false),
            },
            None => Ok(false),
        }
    }

    /// Inserts or updates the given entity, depending on whether it already exists in the database.
    /// Returns false if entity exists and no update is needed
    pub async fn upsert<R: SyncEntityRepository>(&self, value: &R::RepoEntity) -> Result<bool> {
        let mut conn = self
            .sql
            .acquire()
            .await
            .map_err(DsotDatabaseError::DatabaseError)?;
        let op = match R::try_get(&mut *conn, value.get_id()).await? {
            Some(prev) => match value.op_update(&prev) {
                Some(op) => op,
                None => return Ok(false),
            },
            None => value.op_create()?,
        };
        self.exec_op::<R>(op).await?;
        Ok(true)
    }

    /// Lists entities in the database, starting from the given offset and returning up to the given count. Returns an error if retrieval fails.
    pub async fn list<R: SyncEntityRepository>(
        &self,
        count: i64,
        offset: i64,
    ) -> Result<Vec<R::RepoEntity>> {
        let mut conn = self
            .sql
            .acquire()
            .await
            .map_err(DsotDatabaseError::DatabaseError)?;
        let res = R::list(&mut *conn, ListQuery { count, offset }).await?;

        Ok(res)
    }

    /// Deletes the entity with the given ID, if exists.
    pub async fn delete<R: SyncEntityRepository>(&self, id: uuid::Uuid) -> Result<()> {
        let op = SyncOperation::Delete(id);
        self.exec_op::<R>(op).await?;
        Ok(())
    }

    /// Restores a deleted entity with the given ID, if exists.
    pub async fn restore<R: SyncEntityRepository>(&self, id: uuid::Uuid) -> Result<()> {
        let op = SyncOperation::Restore(id);
        self.exec_op::<R>(op).await?;
        Ok(())
    }

    /// Add entry to journal and updates current database
    pub async fn apply_journal<R: SyncEntityRepository>(
        &self,
        entry: JournalEntry,
    ) -> Result<Uuid> {
        let id = entry.id;
        let op = entry.op.clone();

        if &entry.table != R::get_table_name() {
            return Err(DsotDatabaseError::TableMissmatchError(
                entry.table,
                R::get_table_name(),
            ));
        }

        let bytes = entry.to_bytes()?;
        let jrn_trx = self.journal.begin_write()?;
        {
            let mut table = jrn_trx.open_table(JOURNAL_TABLE)?;
            table.insert(id.to_bytes_le(), bytes.as_slice())?;
        }

        let mut sql_trx = self.sql.begin().await?;
        R::exec_op(&mut *sql_trx, op).await?;

        jrn_trx.commit()?;
        match sql_trx.commit().await {
            Ok(_) => Ok(id),
            Err(e) => {
                self.remove_journal_entry(id)?;
                Err(DsotDatabaseError::DatabaseError(e))
            }
        }
    }

    pub(crate) async fn exec_op<R: SyncEntityRepository>(&self, op: SyncOperation) -> Result<Uuid> {
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
}
