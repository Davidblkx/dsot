use std::path::Path;
use dsot_core::{storage::StorageProvider, error::{DsotError, Result}};
use super::RedbStorage;

/// Storage provider for Redb databases stored in a file.
pub struct RedbFileProvider;

impl RedbFileProvider {
    pub fn new() -> Self {
        Self
    }

    /// Create a new Redb database stored in a file.
    pub fn create(path: impl AsRef<Path>) -> Result<RedbStorage> {
        Self::new().open(path)
    }
}

impl StorageProvider for RedbFileProvider {
    type T = RedbStorage;

    /// Open a Redb database stored in a file.
    ///
    /// If the file does not exist, it will be created.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file.
    fn open(&self, path: impl AsRef<Path>) -> Result<Self::T> {
        let err_path = path.as_ref().to_str().unwrap_or("[unknown_path]").to_string();
        let db = redb::Database::create(path)
            .map_err(|e| DsotError::OpenDatabaseError(
                err_path,
                e.to_string()
            ))?;

        Ok(RedbStorage::new(db))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_open_file() {
        let dir = tempdir().unwrap();
        let path = PathBuf::from(dir.path()).join("test.db");
        let provider = RedbFileProvider::new();
        let _storage = provider.open(&path).unwrap();
        assert!(path.exists());
    }
}
