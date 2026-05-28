use std::path::PathBuf;

mod backup;
mod error;
mod load;

pub use backup::DatabaseBackup;
pub use error::{DatabaseManagerError, Result};

#[derive(Debug)]
pub struct DatabaseManager {
    pub(crate) dir: PathBuf,
}

impl DatabaseManager {
    pub fn open_folder<P: Into<PathBuf>>(folder: P) -> Result<Self> {
        let dir = folder.into();

        if !dir.exists() {
            std::fs::create_dir_all(&dir)?;
        }

        if !dir.is_dir() {
            return Err(DatabaseManagerError::PathIsNotAFolder);
        }

        Ok(DatabaseManager { dir })
    }
}
