use std::path::Path;

use super::RedbHandler;
use dsot_core::error::{DsotError, Result};

static MEMORY_NAME : &'static str = "[IN MEMORY]";

pub struct RedbStorage;

impl RedbStorage {
    pub fn open_file(path: impl AsRef<Path>) -> Result<RedbHandler> {
        let err_path = path.as_ref().to_str().unwrap_or("[unknown_path]").to_string();
        let db = redb::Database::create(path)
            .map_err(|e| DsotError::OpenDatabaseError(
                err_path,
                e.to_string()
            ))?;

        Ok(RedbHandler::new(db))
    }

    pub fn open_memory() -> Result<RedbHandler> {
        let db = redb::Database::builder()
            .create_with_backend(redb::backends::InMemoryBackend::default())
            .map_err(|e| DsotError::OpenDatabaseError(
                MEMORY_NAME.to_string(),
                e.to_string()
            ))?;
        Ok(RedbHandler::new(db))
    }
}
