use redb::{ReadableTable, WriteTransaction};
use dsot_core::storage::StorageTransaction;
use dsot_core::error::{DsotError, Result};

pub struct RedbTransaction {
    pub name: String,
    pub trx: WriteTransaction,
}

impl RedbTransaction {
    pub fn new(name: &str, trx: WriteTransaction) -> Self {
        Self { name: name.to_string(), trx }
    }
}

impl StorageTransaction for RedbTransaction {
    fn has(&self, key: &[u8]) -> Result<bool> {
        let table_def = create_table_def!(self.name);

        self.trx
            .open_table(table_def)
            .map_err(to_trx_err!(self.name, "Open table"))?
            .get(key)
            .map_err(to_trx_err!(self.name, "Read value"))
            .map(|v| v.is_some())
    }

    fn list(&self) -> Result<Vec<(Vec<u8>, Vec<u8>)>> {
        let table_def = create_table_def!(self.name);

        self.trx
            .open_table(table_def)
            .map_err(to_trx_err!(self.name, "Open table"))?
            .iter()
            .map_err(to_trx_err!(self.name, "Read list of values"))
            .map(|mut iter| {
                let mut values = Vec::new();

                while let Some(item) = iter.next() {
                    let (key, value) = item.map_err(to_trx_err!(self.name, "Read value"))?;
                    values.push((key.value().to_vec(), value.value().to_vec()));
                }

                Ok(values)
            })?
    }

    fn lookup(&self, predicate: fn(key: &[u8], value: &[u8]) -> bool) -> Result<Option<Vec<u8>>> {
        let table_def = create_table_def!(self.name);

        let table = self.trx
            .open_table(table_def)
            .map_err(to_trx_err!(self.name, "Open table"))?;

        for item in table.iter().map_err(to_trx_err!(self.name, "Read list of values"))? {
            let (key, value) = item.map_err(to_trx_err!(self.name, "Read value"))?;
            if predicate(key.value(), value.value()) {
                return Ok(Some(value.value().to_vec()));
            }
        }

        Ok(None)
    }

    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        let table_def = create_table_def!(self.name.as_str());

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

    fn remove(&mut self, key: &[u8]) -> Result<()> {
        let table_def = create_table_def!(self.name);

        self.trx
            .open_table(table_def)
            .map_err(to_trx_err!(self.name, "Open table"))?
            .remove(key)
            .map_err(to_trx_err!(self.name, "Remove value"))?;

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
}

#[cfg(test)]
mod tests {
    use dsot_core::storage::StorageHandler;
    use super::super::RedbStorage;
    use super::*;

    #[test]
    fn set_value() {
        let storage = RedbStorage::open_memory().unwrap();

        let mut trx = storage.open("test").unwrap();
        trx.set(b"key", b"value").unwrap();
        trx.commit().unwrap();

        let trx = storage.open("test").unwrap();
        let value = trx.get(b"key").unwrap().unwrap();
        trx.close().unwrap();
        assert_eq!(value, b"value");

        let mut trx = storage.open("test").unwrap();
        trx.set(b"key", b"new_value").unwrap();
        trx.commit().unwrap();

        let trx = storage.open("test").unwrap();
        let value = trx.get(b"key").unwrap().unwrap();
        trx.close().unwrap();
        assert_eq!(value, b"new_value");
    }

    #[test]
    fn check_for_key() {
        let storage = RedbStorage::open_memory().unwrap();
        let mut trx = storage.open("test").unwrap();

        assert!(!trx.has(b"key").unwrap());

        trx.set(b"key", b"value").unwrap();
        trx.commit().unwrap();

        let trx = storage.open("test").unwrap();
        assert!(trx.has(b"key").unwrap());
    }

    #[test]
    fn rollback_transaction() {
        let storage = RedbStorage::open_memory().unwrap();
        let mut trx = storage.open("test").unwrap();

        trx.set(b"key", b"value").unwrap();
        trx.commit().unwrap();

        let mut trx = storage.open("test").unwrap();
        trx.set(b"key", b"new_value").unwrap();
        trx.rollback().unwrap();

        let trx = storage.open("test").unwrap();
        let value = trx.get(b"key").unwrap().unwrap();
        assert_eq!(value, b"value");
    }

    #[test]
    fn remove_key() {
        let storage = RedbStorage::open_memory().unwrap();
        let mut trx = storage.open("test").unwrap();

        trx.set(b"key", b"value").unwrap();
        trx.commit().unwrap();

        let trx = storage.open("test").unwrap();
        assert!(trx.has(b"key").unwrap());
        trx.close().unwrap();

        let mut trx = storage.open("test").unwrap();
        trx.remove(b"key").unwrap();
        trx.commit().unwrap();

        let trx = storage.open("test").unwrap();
        assert!(!trx.has(b"key").unwrap());
    }

    #[test]
    fn list_entries() {
        let storage = RedbStorage::open_memory().unwrap();
        let mut trx = storage.open("test").unwrap();

        trx.set(b"key1", b"value1").unwrap();
        trx.set(b"key2", b"value2").unwrap();
        trx.commit().unwrap();

        let trx = storage.open("test").unwrap();
        let entries = trx.list().unwrap();
        trx.close().unwrap();

        assert_eq!(entries.len(), 2);
        assert!(entries.contains(&(b"key1".to_vec(), b"value1".to_vec())));
        assert!(entries.contains(&(b"key2".to_vec(), b"value2".to_vec())));
    }

    #[test]
    fn lookup_entry() {
        let storage = RedbStorage::open_memory().unwrap();
        let mut trx = storage.open("test").unwrap();

        trx.set(b"key1", b"value1").unwrap();
        trx.set(b"key2", b"value2").unwrap();
        trx.commit().unwrap();

        let trx = storage.open("test").unwrap();
        let value = trx.lookup(|_, v| v == b"value2").unwrap();
        trx.close().unwrap();

        assert_eq!(value.unwrap(), b"value2");
    }

    #[test]
    fn list_in_order() {
        let storage = RedbStorage::open_memory().unwrap();
        let mut trx = storage.open("test").unwrap();

        trx.set(b"key2", b"value2").unwrap();
        trx.set(b"key1", b"value1").unwrap();
        trx.commit().unwrap();

        let trx = storage.open("test").unwrap();
        let entries = trx.list().unwrap();
        trx.close().unwrap();

        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0], (b"key1".to_vec(), b"value1".to_vec()));
        assert_eq!(entries[1], (b"key2".to_vec(), b"value2".to_vec()));
    }
}
