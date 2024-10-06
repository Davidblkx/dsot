use std::path::{Path, PathBuf};

use redb::ReadableTable;

use crate::error::{DsotError, Result};
use super::{Storage, StorageProvider, TableTransaction};

macro_rules! to_trans_error {
    ($self:ident, $err:expr) => {
        |e| DsotError::TableTransactionError {
            table: $self.name,
            operation: $err,
            error: e.to_string()
        }
    };
}

macro_rules! table_version_error {
    ($name:ident, $err:expr) => {
        |e| DsotError::TableTransactionError {
            table: "table_versions",
            operation: $err,
            error: e.to_string()
        }
    };
}

pub struct RedbTransaction {
    pub trx: redb::WriteTransaction,
    pub name: &'static str,
}

impl RedbTransaction {
    pub fn new(trx: redb::WriteTransaction, table: &'static str) -> Self {
        Self { trx, name: table }
    }
}

impl TableTransaction for RedbTransaction {
    fn has(&self, key: &[u8]) -> Result<bool> {
        let table_def: redb::TableDefinition<&[u8], &[u8]> = redb::TableDefinition::new(self.name);
        let has = {
            self.trx
                .open_table(table_def)
                .map_err(to_trans_error!(self, "Open table"))?
                .get(key)
                .map_err(to_trans_error!(self, "Check if key exist"))?.is_some()
        };
        Ok(has)
    }

    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        let table_def: redb::TableDefinition<&[u8], &[u8]> = redb::TableDefinition::new(self.name);
        let has = {
            self.trx
                .open_table(table_def)
                .map_err(to_trans_error!(self, "Open table"))?.get(key)
                .map_err(to_trans_error!(self, "Read value"))?.map(|v| v.value().to_vec())
        };
        Ok(has)
    }

    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        let table_def: redb::TableDefinition<&[u8], &[u8]> = redb::TableDefinition::new(self.name);
        {
            self.trx
                .open_table(table_def)
                .map_err(to_trans_error!(self, "Open table"))?
                .insert(key, value)
                .map_err(to_trans_error!(self, "Insert value"))?
        };
        Ok(())
    }

    fn commit(self) -> Result<()> {
        self.trx.commit()
            .map_err(to_trans_error!(self, "Commit transaction"))?;
        Ok(())
    }

    fn rollback(self) -> Result<()> {
        self.trx.abort()
            .map_err(to_trans_error!(self, "Rollback transaction"))?;
        Ok(())
    }

    fn close(self) -> Result<()> {
        Ok(())
    }

    fn is_open(&self) -> bool {
        true
    }
}

pub struct RedbStorage {
    pub db: redb::Database,
}

impl RedbStorage {
    pub fn new(db: redb::Database) -> Self {
        Self { db }
    }
}

impl Storage for RedbStorage {
    type T = RedbTransaction;

    fn get_table_version(&self, table_name: &str) -> Result<u64> {
        let table_def: redb::TableDefinition<&str, u64> = redb::TableDefinition::new("table_versions");
        let version = {
            let read_trx = self.db.begin_read()
                .map_err(table_version_error!(table_name, "Begin read transaction"))?;
            read_trx.open_table(table_def)
                .map_err(table_version_error!(table_name, "Open table"))?
                .get(table_name)
                .map_err(table_version_error!(table_name, "Read table version"))?
                .map(|v| v.value())
        }
        .unwrap_or(0);

        Ok(version)
    }

    fn set_table_version(&self, table_name: &str, version: u64) -> Result<()> {
        let table_def: redb::TableDefinition<&str, u64> = redb::TableDefinition::new("table_versions");
        {
            let write_trx = self.db
                .begin_write()
                .map_err(table_version_error!(table_name, "Begin write transaction"))?;
            write_trx.open_table(table_def)
                .map_err(table_version_error!(table_name, "Open table"))?
                .insert(table_name, version)
                .map_err(table_version_error!(table_name, "Insert table version"))?;
            write_trx.commit()
                .map_err(table_version_error!(table_name, "Commit transaction"))?;
        }
        Ok(())
    }

    fn open_table(&self, table_name: &'static str) -> Result<Self::T> {
        let trx = self.db.begin_write()
            .map_err(|e| DsotError::TableTransactionError {
                table: table_name,
                operation: "Create write transaction",
                error: e.to_string()
            })?;

        Ok(RedbTransaction::new(trx, table_name))
    }
}

pub struct RedbFileProvider {
    pub path: PathBuf,
}

impl RedbFileProvider {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self { path: path.as_ref().to_path_buf() }
    }

    pub fn create(path: impl AsRef<Path>) -> Result<impl Storage> {
        Self::new(path).open()
    }
}

impl StorageProvider for RedbFileProvider {
    type T = RedbStorage;

    fn open(&self) -> Result<Self::T> {
        let db = redb::Database::open(self.path.clone())
            .map_err(|e| DsotError::OpenDatabaseError(
                self.path.to_str().unwrap_or("[unknown_path]").to_string(),
                e.to_string()
            ))?;

        Ok(RedbStorage::new(db))
    }
}

pub struct RedbMemoryProvider;

impl StorageProvider for RedbMemoryProvider {
    type T = RedbStorage;

    fn open(&self) -> Result<Self::T> {
        let db = redb::Database::builder()
            .create_with_backend(redb::backends::InMemoryBackend::default())
            .map_err(|e| DsotError::OpenDatabaseError(
                "memory".to_string(),
                e.to_string()
            ))?;

        Ok(RedbStorage::new(db))
    }
}
