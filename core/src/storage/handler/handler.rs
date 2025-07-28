use core::result::Result::Ok;
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::path::PathBuf;
use std::str::FromStr;

use crate::error::Result;
use crate::model::JournalEntry;
use crate::storage::SqlTransaction;
use crate::storage::kv::redb::{RedbHandler, RedbStorage, RedbTransaction};
use crate::storage::{StorageEntity, StorageHandler};

use super::HandlerConnectionKind;
use super::error::DatabaseHandlerError;

/// Handles a database IO and allows to sync/backup
/// It will also handle any required migration
pub struct SqliteDbHandler {
    /// Defines where data is loaded from (memory or file)
    pub connection_kind: HandlerConnectionKind,
    pub(crate) db_pool: Option<SqlitePool>,
    pub(crate) journal: Option<RedbHandler>,
}

impl SqliteDbHandler {
    /// Create a new handler for an in memory database
    pub fn new_memory() -> Self {
        Self {
            connection_kind: HandlerConnectionKind::Memory,
            db_pool: None,
            journal: None,
        }
    }

    pub fn new_file<N: Into<String>, P: Into<PathBuf>>(name: N, data_folder: P) -> Self {
        Self {
            connection_kind: HandlerConnectionKind::new_file(name, data_folder),
            db_pool: None,
            journal: None,
        }
    }

    pub fn new_file_with_backup<N: Into<String>, P: Into<PathBuf>, B: Into<PathBuf>>(
        name: N,
        data_folder: P,
        backup_folder: B,
    ) -> Self {
        Self {
            connection_kind: HandlerConnectionKind::new_file(name, data_folder)
                .with_backup_folder(backup_folder),
            db_pool: None,
            journal: None,
        }
    }

    /// Opens the connection to the database and it's journal
    pub async fn open(&mut self) -> Result<()> {
        if self.is_open() {
            return Ok(());
        }

        match &self.connection_kind {
            HandlerConnectionKind::Memory => self.open_memory().await,
            _ => {
                if let (Some(db_path), Some(journal_path)) = (
                    self.connection_kind.get_db_path(),
                    self.connection_kind.get_journal_path(),
                ) {
                    return self.open_file(db_path, journal_path).await;
                } else {
                    return DatabaseHandlerError::PathNotAvailable.to_err();
                }
            }
        }
    }

    /// Open database connection to a in memory database
    async fn open_memory(&mut self) -> Result<()> {
        log::trace!("Openning connection to memory database");
        self.journal = Some(RedbStorage::open_memory()?);

        let conn = SqliteConnectOptions::from_str("sqlite://memory")?.create_if_missing(true);
        let pool = SqlitePool::connect_with(conn).await?;
        self.db_pool = Some(pool);

        Ok(())
    }

    /// Open database connection to a file
    async fn open_file(&mut self, db_path: PathBuf, journal_path: PathBuf) -> Result<()> {
        log::trace!(
            "Opening connection to journal db: {}",
            journal_path.display()
        );
        self.journal = Some(RedbStorage::open_file(journal_path)?);

        let conn_str = format!("sqlite://{}", db_path.display());
        let conn = SqliteConnectOptions::from_str(&conn_str)?.create_if_missing(true);
        let pool = SqlitePool::connect_with(conn).await?;
        self.db_pool = Some(pool);

        Ok(())
    }

    /// Checks if connection is open
    pub fn is_open(&self) -> bool {
        if let (Some(_), Some(_)) = (&self.db_pool, &self.journal) {
            true
        } else {
            false
        }
    }

    /// Close connection
    pub async fn close(&mut self) -> Result<()> {
        if !self.is_open() {
            return Ok(());
        }

        if let Some(pool) = &self.db_pool {
            if !pool.is_closed() {
                pool.close().await;
            }
        }

        self.db_pool = None;
        self.journal = None;

        Ok(())
    }

    pub fn ensure_connection(&self) -> Result<()> {
        if self.is_open() {
            Ok(())
        } else {
            DatabaseHandlerError::DatabaseConnectionUnavailable.to_err()
        }
    }

    pub fn create_journal_transaction(&self) -> Result<RedbTransaction> {
        self.ensure_connection()?;
        self.journal
            .as_ref()
            .unwrap()
            .open(JournalEntry::get_storage_name())
    }

    pub async fn create_db_transaction(&self) -> Result<SqlTransaction> {
        self.ensure_connection()?;
        let pool: &sqlx::SqlitePool = self.db_pool.as_ref().unwrap();
        let trx = pool.begin().await?;

        Ok(trx)
    }
}
