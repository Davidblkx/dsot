use core::result::Result::Ok;
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::path::PathBuf;
use std::str::FromStr;

use crate::error::Result;
use crate::storage::kv::redb::{RedbHandler, RedbStorage};

pub enum HandlerKind {
    Memory,
    File(PathBuf),
}

/// Handles a database IO and allows to sync/backup
/// It will also handle any required migration
pub struct SqliteDbHandler {
    // Defines where data is loaded from (memory or file)
    pub kind: HandlerKind,
    pub(crate) db_pool: Option<SqlitePool>,
    pub(crate) journal: Option<RedbHandler>,
}

impl SqliteDbHandler {
    /// Opens the connection to the database and it's journal
    pub async fn open(&mut self) -> Result<()> {
        if self.is_open() {
            return Ok(());
        }

        match &self.kind {
            HandlerKind::Memory => self.open_memory().await,
            HandlerKind::File(path) => self.open_file(path.clone()).await,
        }
    }

    async fn open_memory(&mut self) -> Result<()> {
        log::trace!("Openning connection to memomry database");
        self.journal = Some(RedbStorage::open_memory()?);

        let conn = SqliteConnectOptions::from_str("sqlite://memory")?.create_if_missing(true);
        let pool = SqlitePool::connect_with(conn).await?;
        self.db_pool = Some(pool);

        Ok(())
    }

    async fn open_file(&mut self, path: PathBuf) -> Result<()> {
        // TODO: logs
        let mut jrn_path = path.clone();
        jrn_path.set_extension("journal");
        self.journal = Some(RedbStorage::open_file(jrn_path)?);

        let conn_str = format!("sqlite://{}", path.display());
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
}
