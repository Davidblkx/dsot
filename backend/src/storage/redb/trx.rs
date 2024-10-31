use dsot_core::{storage::TableTransaction, error::{DsotError, Result}};
use redb::{ReadableTable, WriteTransaction};

/// A transaction for a Redb table.
///
/// This struct is a wrapper around a `redb::WriteTransaction` and a table name.
/// It implements the `TableTransaction` trait for the Redb storage provider.
pub struct RedbTransaction {
    /// The Redb transaction.
    pub trx: WriteTransaction,
    /// The name of the table, it must be a static string.
    pub name: &'static str,
}

impl RedbTransaction {
    /// Create a new Redb transaction.
    ///
    /// # Arguments
    ///
    /// * `trx` - The Redb transaction.
    /// * `table` - The name of the table.
    ///
    /// # Returns
    /// A new Redb transaction.
    pub fn new(trx: WriteTransaction, table: &'static str) -> Self {
        Self { trx, name: table }
    }
}

impl TableTransaction for RedbTransaction {
    fn has(&self, key: &[u8]) -> Result<bool> {
        let table_def = create_table_def!(self.name);

        self.trx
            .open_table(table_def)
            .map_err(to_trx_err!(self.name, "Open table"))?
            .get(key)
            .map_err(to_trx_err!(self.name, "Read value"))
            .map(|v| v.is_some())
    }

    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        let table_def = create_table_def!(self.name);

        self.trx
            .open_table(table_def)
            .map_err(to_trx_err!(self.name, "Open table"))?
            .get(key)
            .map_err(to_trx_err!(self.name, "Read value"))
            .map(|v| v.map(|v| v.value().to_vec()))
    }

    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        let table_def = create_table_def!(self.name);

        self.trx
            .open_table(table_def)
            .map_err(to_trx_err!(self.name, "Open table"))?
            .insert(key, value)
            .map_err(to_trx_err!(self.name, "Insert value"))?;

        Ok(())
    }

    fn commit(self) -> Result<()> {
        self.trx.commit()
            .map_err(to_trx_err!(self.name, "Commit transaction"))?;
        Ok(())
    }

    fn rollback(self) -> Result<()> {
        self.trx.abort()
            .map_err(to_trx_err!(self.name, "Rollback transaction"))?;
        Ok(())
    }

    fn close(self) -> Result<()> {
        Ok(())
    }

    fn is_open(&self) -> bool {
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use dsot_core::storage::Storage;

    use crate::storage::redb::RedbInMemoryProvider;

    static TABLE_NAME : &'static str = "TEST_TABLE";

    #[test]
    fn set_value() {
        let storage = RedbInMemoryProvider::create().unwrap();

        let mut trx = storage.open_table(TABLE_NAME).unwrap();
        trx.set(b"key", b"value").unwrap();
        trx.commit().unwrap();

        let trx = storage.open_table(TABLE_NAME).unwrap();
        let value = trx.get(b"key").unwrap().unwrap();
        trx.close().unwrap();
        assert_eq!(value, b"value");

        let mut trx = storage.open_table(TABLE_NAME).unwrap();
        trx.set(b"key", b"new_value").unwrap();
        trx.commit().unwrap();

        let trx = storage.open_table(TABLE_NAME).unwrap();
        let value = trx.get(b"key").unwrap().unwrap();
        trx.close().unwrap();
        assert_eq!(value, b"new_value");
    }

    #[test]
    fn check_for_key() {
        let storage = RedbInMemoryProvider::create().unwrap();
        let mut trx = storage.open_table(TABLE_NAME).unwrap();

        assert!(!trx.has(b"key").unwrap());

        trx.set(b"key", b"value").unwrap();
        trx.commit().unwrap();

        let trx = storage.open_table(TABLE_NAME).unwrap();
        assert!(trx.has(b"key").unwrap());
    }

    #[test]
    fn rollback_transaction() {
        let storage = RedbInMemoryProvider::create().unwrap();
        let mut trx = storage.open_table(TABLE_NAME).unwrap();

        trx.set(b"key", b"value").unwrap();
        trx.commit().unwrap();

        let mut trx = storage.open_table(TABLE_NAME).unwrap();
        trx.set(b"key", b"new_value").unwrap();
        trx.rollback().unwrap();

        let trx = storage.open_table(TABLE_NAME).unwrap();
        let value = trx.get(b"key").unwrap().unwrap();
        assert_eq!(value, b"value");
    }
}
