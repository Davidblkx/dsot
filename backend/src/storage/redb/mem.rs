use std::path::Path;
use dsot_core::{storage::StorageProvider, error::{DsotError, Result}};
use super::RedbStorage;

static NAME : &'static str = "[IN MEMORY]";

pub struct RedbInMemoryProvider;

impl RedbInMemoryProvider {
    pub fn new() -> Self {
        Self
    }

    pub fn create() -> Result<RedbStorage> {
        Self::new().open(NAME)
    }
}

impl StorageProvider for RedbInMemoryProvider {
    type T = RedbStorage;

    fn open(&self, _path: impl AsRef<Path>) -> Result<Self::T> {
        let db = redb::Database::builder()
            .create_with_backend(redb::backends::InMemoryBackend::default())
            .map_err(|e| DsotError::OpenDatabaseError(
                NAME.to_string(),
                e.to_string()
            ))?;

        Ok(RedbStorage::new(db))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_memory() {
        let provider = RedbInMemoryProvider::new();
        let _storage = provider.open(NAME).unwrap();
    }
}
