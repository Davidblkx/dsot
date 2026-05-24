use uuid::Uuid;

use crate::entity::SyncEntity;
use crate::model::{JournalEntry, SyncOperation};
use crate::repo::{ListQuery, SyncEntityRepository};

use super::{
    DsotDatabase, DsotDatabaseError, DsotDatabaseTransaction, Result, journal::JOURNAL_TABLE,
};

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
        let mut trx = self.begin_transaction().await?;
        trx.insert::<R>(value).await?;
        trx.commit().await?;
        Ok(())
    }

    /// Updates the given entity if it exists and has changes. Returns false if entity does not exist or no update is needed.
    pub async fn update<R: SyncEntityRepository>(&self, value: &R::RepoEntity) -> Result<bool> {
        let mut trx = self.begin_transaction().await?;
        let res = trx.update::<R>(value).await?;
        trx.commit().await?;
        Ok(res)
    }

    /// Inserts or updates the given entity, depending on whether it already exists in the database.
    /// Returns false if entity exists and no update is needed
    pub async fn upsert<R: SyncEntityRepository>(&self, value: &R::RepoEntity) -> Result<bool> {
        let mut trx = self.begin_transaction().await?;
        let res = trx.upsert::<R>(value).await?;
        trx.commit().await?;
        Ok(res)
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
        let mut trx = self.begin_transaction().await?;
        trx.delete::<R>(id).await?;
        trx.commit().await?;
        Ok(())
    }

    /// Restores a deleted entity with the given ID, if exists.
    pub async fn restore<R: SyncEntityRepository>(&self, id: uuid::Uuid) -> Result<()> {
        let mut trx = self.begin_transaction().await?;
        trx.restore::<R>(id).await?;
        trx.commit().await?;
        Ok(())
    }

    /// Search for entity using https://sqlite.org/fts5.html
    pub async fn search<R: SyncEntityRepository>(&self, query: &str) -> Result<Vec<R::RepoEntity>> {
        let mut conn = self.sql.acquire().await?;
        let list = R::search(&mut *conn, query.to_string()).await?;
        Ok(list)
    }

    /// Add entry to journal and updates current database
    pub async fn apply_journal<R: SyncEntityRepository>(
        &self,
        entry: JournalEntry,
    ) -> Result<Uuid> {
        let id = entry.id;
        let mut trx = self.begin_transaction().await?;
        trx.apply_journal::<R>(entry).await?;
        match trx.commit().await {
            Ok(_) => Ok(id),
            Err(e) => {
                self.remove_journal_entry(id)?;
                Err(e)
            }
        }
    }
}

impl<'a> DsotDatabaseTransaction<'a> {
    /// Tries to get the entity with the given ID. Returns None if entity does not exist, or an error if retrieval fails.
    pub async fn try_get<R: SyncEntityRepository>(
        &mut self,
        id: uuid::Uuid,
    ) -> Result<Option<R::RepoEntity>> {
        let row = R::try_get(&mut *self.sql_trx, id).await?;
        Ok(row)
    }

    /// Gets the entity with the given ID. Returns an error if entity does not exist or retrieval fails.
    pub async fn get<R: SyncEntityRepository>(&mut self, id: uuid::Uuid) -> Result<R::RepoEntity> {
        let row = R::get(&mut *self.sql_trx, id).await?;
        Ok(row)
    }

    /// Inserts the given entity. Returns an error if entity already exists or if creation fails.
    pub async fn insert<R: SyncEntityRepository>(&mut self, value: &R::RepoEntity) -> Result<()> {
        let op = value.op_create()?;
        self.create_journal_and_apply_op::<R>(op).await?;
        Ok(())
    }

    /// Updates the given entity if it exists and has changes. Returns false if entity does not exist or no update is needed.
    pub async fn update<R: SyncEntityRepository>(&mut self, value: &R::RepoEntity) -> Result<bool> {
        match R::try_get(&mut *self.sql_trx, value.get_id()).await? {
            Some(prev) => match value.op_update(&prev) {
                Some(op) => {
                    self.create_journal_and_apply_op::<R>(op).await?;
                    Ok(true)
                }
                None => Ok(false),
            },
            None => Ok(false),
        }
    }

    /// Inserts or updates the given entity, depending on whether it already exists in the database.
    /// Returns false if entity exists and no update is needed
    pub async fn upsert<R: SyncEntityRepository>(&mut self, value: &R::RepoEntity) -> Result<bool> {
        let op = match R::try_get(&mut *self.sql_trx, value.get_id()).await? {
            Some(prev) => match value.op_update(&prev) {
                Some(op) => op,
                None => return Ok(false),
            },
            None => value.op_create()?,
        };
        self.create_journal_and_apply_op::<R>(op).await?;
        Ok(true)
    }

    /// Lists entities in the database, starting from the given offset and returning up to the given count. Returns an error if retrieval fails.
    pub async fn list<R: SyncEntityRepository>(
        &mut self,
        count: i64,
        offset: i64,
    ) -> Result<Vec<R::RepoEntity>> {
        let res = R::list(&mut *self.sql_trx, ListQuery { count, offset }).await?;
        Ok(res)
    }

    /// Deletes the entity with the given ID, if exists.
    pub async fn delete<R: SyncEntityRepository>(&mut self, id: uuid::Uuid) -> Result<()> {
        let op = SyncOperation::Delete(id);
        self.create_journal_and_apply_op::<R>(op).await?;
        Ok(())
    }

    /// Restores a deleted entity with the given ID, if exists.
    pub async fn restore<R: SyncEntityRepository>(&mut self, id: uuid::Uuid) -> Result<()> {
        let op = SyncOperation::Restore(id);
        self.create_journal_and_apply_op::<R>(op).await?;
        Ok(())
    }

    /// Search for entity using https://sqlite.org/fts5.html
    pub async fn search<R: SyncEntityRepository>(
        &mut self,
        query: &str,
    ) -> Result<Vec<R::RepoEntity>> {
        let list = R::search(&mut *self.sql_trx, query.to_string()).await?;
        Ok(list)
    }

    /// Add entry to journal and updates current database
    pub async fn apply_journal<R: SyncEntityRepository>(
        &mut self,
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
        {
            let mut table = self.journal_trx.open_table(JOURNAL_TABLE)?;
            table.insert(id.to_bytes_le(), bytes.as_slice())?;
        }

        self.safe_apply_op::<R>(op).await?;

        Ok(id)
    }

    /// Apply table operation, when create, skips if entity already exists
    /// This doesn't add operation to journal and could cause sync problems
    pub async fn safe_apply_op<R: SyncEntityRepository>(
        &mut self,
        op: SyncOperation,
    ) -> Result<()> {
        let mut should_exec = true;
        if let SyncOperation::Create(ref data) = op {
            let entity = <R::RepoEntity as SyncEntity>::from_bytes(data)?;
            if R::try_get(&mut *self.sql_trx, entity.get_id())
                .await?
                .is_some()
            {
                should_exec = false;
            }
        }

        if should_exec {
            R::exec_op(&mut *self.sql_trx, op).await?;
        }

        Ok(())
    }

    pub(crate) async fn create_journal_and_apply_op<R: SyncEntityRepository>(
        &mut self,
        op: SyncOperation,
    ) -> Result<Uuid> {
        let (jrn_id, jrn_bytes) = JournalEntry::create_entry(R::get_table_name(), &op)?;
        {
            let mut table = self.journal_trx.open_table(JOURNAL_TABLE)?;
            table.insert(jrn_id.to_bytes_le(), jrn_bytes.as_slice())?;
        }

        R::exec_op(&mut *self.sql_trx, op).await?;
        Ok(jrn_id)
    }
}
