use crate::error::Result;

pub trait StorageProvider {
    type T: Storage;
    fn open(&self) -> Result<Self::T>;
}

pub trait Storage {
    type T: TableTransaction;
    fn get_table_version(&self, table_name: &str) -> Result<u64>;
    fn set_table_version(&self, table_name: &str, version: u64) -> Result<()>;
    fn open_table(&self, table_name: &'static str) -> Result<Self::T>;
}

pub trait TableTransaction {
    fn has(&self, key: &[u8]) -> Result<bool>;
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>>;
    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<()>;
    fn commit(self) -> Result<()>;
    fn rollback(self) -> Result<()>;
    fn close(self) -> Result<()>;
    fn is_open(&self) -> bool;
}
