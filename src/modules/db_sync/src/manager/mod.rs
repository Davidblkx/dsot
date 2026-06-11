use std::{fmt::Debug, path::PathBuf};

mod backup;
mod load;

pub use backup::DatabaseBackup;

pub type DatabaseManagerError = crate::DBSyncError;
pub type Result<T> = std::result::Result<T, crate::DBSyncError>;

#[derive(Debug, Clone)]
pub struct DatabaseManager {
    pub(crate) dir: PathBuf,
}

pub trait DatabaseManagerProvider {
    fn provide(&self, id: &str) -> Result<DatabaseManager>;
}

impl DatabaseManager {
    pub fn open_folder<P: Into<PathBuf>>(folder: P) -> Result<Self> {
        let dir = folder.into();
        log::debug!("Opening database manager folder: {}", dir.display());

        if !dir.exists() {
            log::info!("Creating database folder: {}", dir.display());
            std::fs::create_dir_all(&dir)?;
        }

        if !dir.is_dir() {
            log::error!("Path is not a directory: {}", dir.display());
            return Err(DatabaseManagerError::PathIsNotAFolder);
        }

        Ok(DatabaseManager { dir })
    }
}
