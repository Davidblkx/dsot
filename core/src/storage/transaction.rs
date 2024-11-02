use crate::error::Result;

/// Defines the operations that can be performed on a storage bucket.
pub trait StorageTransaction {
    /// Check if a key exists.
    fn has(&self, key: &[u8]) -> Result<bool>;
    /// List all objects.
    fn list(&self) -> Result<Vec<(Vec<u8>, Vec<u8>)>>;
    /// Lookup a value by a predicate.
    fn lookup(&self, predicate: fn(key: &[u8], value: &[u8]) -> bool) -> Result<Option<Vec<u8>>>;
    /// Get the value of a key.
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>>;
    /// Set the value of a key.
    ///
    /// If the key already exists, the value is updated.
    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<()>;
    /// Remove a key.
    fn remove(&mut self, key: &[u8]) -> Result<()>;
    /// Commit the transaction. It consumes the transaction.
    fn commit(self) -> Result<()>;
    /// Rollback the transaction. It consumes the transaction.
    fn rollback(self) -> Result<()>;
    /// Close the transaction. It consumes the transaction.
    fn close(self) -> Result<()>;
}
