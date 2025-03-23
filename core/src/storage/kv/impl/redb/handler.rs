use crate::storage::StorageHandler;
use crate::error::{DsotError, Result};

use super::RedbTransaction;

pub struct RedbHandler {
    pub db: redb::Database,
}

impl RedbHandler {
    pub fn new(db: redb::Database) -> Self {
        Self { db }
    }
}

impl StorageHandler for RedbHandler {
    type Transaction = RedbTransaction;

    fn open(&self, name: &str) -> Result<Self::Transaction> {
        let trx = self.db.begin_write()
            .map_err(to_trx_err!(name.to_string(), "Create write transaction"))?;

        Ok(RedbTransaction::new(name, trx))
    }

    fn remove(&self, name: &str) -> Result<()> {
        let table_def = create_table_def!(name);

        let trx = self.db.begin_write()
            .map_err(to_trx_err!(name.to_string(), "Create write transaction"))?;

        trx.delete_table(table_def)
            .map_err(to_trx_err!(name.to_string(), "Delete table"))?;

        trx.commit()
            .map_err(to_trx_err!(name.to_string(), "Commit transaction"))
    }

    fn exists(&self, name: &str) -> Result<bool> {
        let table_def = create_table_def!(name);

        let open_table = self.db.begin_read()
            .map_err(to_trx_err!(name.to_string(), "Create write transaction"))?
            .open_table(table_def);


        match open_table {
            Ok(_) => Ok(true),
            Err(e) => match e {
                redb::TableError::TableDoesNotExist(_) => Ok(false),
                _ => Err(DsotError::TransactionError {
                    bucket: name.to_string(),
                    operation: "Open table",
                    error: e.to_string()
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::StorageTransaction;
    use super::super::RedbStorage;

    #[test]
    fn test_get() {
        let storage = RedbStorage::open_memory().unwrap();

        let mut bucket = storage.open("test").unwrap();
        assert_eq!(bucket.name, "test");

        bucket.set(b"key", b"value").unwrap();
        bucket.commit().unwrap();

        let bucket = storage.open("test").unwrap();
        let value = bucket.get(b"key").unwrap();

        assert_eq!(value, Some(b"value".to_vec()));
    }

    #[test]
    fn test_exists() {
        let storage = RedbStorage::open_memory().unwrap();

        assert!(!storage.exists("test").unwrap());

        let mut bucket = storage.open("test").unwrap();
        bucket.set(b"key", b"value").unwrap();
        bucket.commit().unwrap();

        assert!(storage.exists("test").unwrap());
    }

    #[test]
    fn test_remove() {
        let storage = RedbStorage::open_memory().unwrap();

        assert!(!storage.exists("test").unwrap());
        storage.remove("test").unwrap();

        let mut bucket = storage.open("test").unwrap();
        bucket.set(b"key", b"value").unwrap();
        bucket.commit().unwrap();

        assert!(storage.exists("test").unwrap());

        storage.remove("test").unwrap();

        assert!(!storage.exists("test").unwrap());
    }
}
