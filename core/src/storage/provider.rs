use crate::error::Result;

pub trait StorageProvider<'a> {
    type T: Storage<'a>;
    fn open(&self) -> Result<Self::T>;
}

pub trait Storage<'a> {
    type T: TableTransaction<'a>;
    fn get_table_version(&self, table_name: &'a str) -> Result<u64>;
    fn set_table_version(&self, table_name: &'a str, version: u64) -> Result<()>;
    fn open_table(&self, table_name: &'a str) -> Result<Self::T>;
}

pub trait TableTransaction<'a> {
    type Iter: Iterator<Item = Result<(&'a [u8], &'a [u8])>>;
    fn iter(&self) -> Result<Self::Iter>;
    fn get(&self, key: &[u8]) -> Result<Option<&'a [u8]>>;
    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<()>;
    fn commit(&self) -> Result<()>;
    fn rollback(&self) -> Result<()>;
    fn close(&mut self) -> Result<()>;
    fn is_open(&self) -> bool;
}
