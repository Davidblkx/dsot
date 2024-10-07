use crate::error::Result;

pub trait StorageProvider {
    type T: Storage;
    fn open(&self, path: impl AsRef<std::path::Path>) -> Result<Self::T>;
}

/// Represents a struct that creates transactions for tables.
pub trait Storage {
    type T: TableTransaction;
    /// Get the version of a table.
    ///
    /// Returns 0 if a version is not found.
    fn get_table_version(&self, table_name: &str) -> Result<u64>;
    /// Set the version of a table.
    fn set_table_version(&self, table_name: &str, version: u64) -> Result<()>;
    /// Open a transaction for a table.
    fn open_table(&self, table_name: &'static str) -> Result<Self::T>;
}

/// A transaction for a table.
///
/// This trait is implemented by the storage provider to provide a common interface for transactions.
pub trait TableTransaction {
    /// Check if a key exists in the table.
    fn has(&self, key: &[u8]) -> Result<bool>;
    /// Get the value of a key in the table.
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>>;
    /// Set the value of a key in the table.
    ///
    /// If the key already exists, the value is updated.
    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<()>;
    /// Commit the transaction. It consumes the transaction.
    fn commit(self) -> Result<()>;
    /// Rollback the transaction. It consumes the transaction.
    fn rollback(self) -> Result<()>;
    /// Close the transaction. It consumes the transaction.
    fn close(self) -> Result<()>;
    /// Check if the transaction is open and can be used.
    fn is_open(&self) -> bool;
}
