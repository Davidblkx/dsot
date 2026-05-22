use redb::Database;
use sqlx::SqlitePool;

mod entity_ops;
mod error;
mod journal;

pub use error::{DsotDatabaseError, Result};

#[derive(Debug)]
pub struct DsotDatabase {
    pub(crate) journal: Database,
    pub(crate) sql: SqlitePool,
}

impl DsotDatabase {
    pub fn new(journal: redb::Database, sql: SqlitePool) -> Self {
        Self { journal, sql }
    }
}
