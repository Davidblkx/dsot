use native_model::Model;

use crate::error::Result;

/// A key for a table in the database.
pub enum TableKey {
    /// A string key.
    String(String),
    /// A UUID7 key.
    Uuid(uuid7::Uuid),
}

pub struct TableIndex {
    pub name: &'static str,
    pub key: TableKey,
}

/// A trait for defining a table in the database.
pub trait Table<T: Model> {
    /// Get the name of the table.
    fn get_name(&self) -> &'static str;
    /// Get the key for the given value.
    fn get_key(&self, value: &T) -> Result<TableKey>;
    /// Create a new unique key for entry.
    fn create_key(&self) -> Result<TableKey>;
}

