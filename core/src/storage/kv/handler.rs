use super::transaction::StorageTransaction;
use crate::error::Result;

/// Defines the operation to access a storage bucket.
pub trait StorageHandler {
    type Transaction: StorageTransaction;

    /// Check if a storage bucket exists.
    fn exists(&self, name: &str) -> Result<bool>;
    /// Get a transaction to access the storage bucket. If it does not exist, it will be created.
    fn open(&self, name: &str) -> Result<Self::Transaction>;
    /// Remove a storage bucket. Operation cannot be undone.
    fn remove(&self, name: &str) -> Result<()>;
}
