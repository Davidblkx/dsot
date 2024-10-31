use crate::error::Result;
use super::{StorageEntry, Migration};

/// This trait is used to serialize/deserialize items and map them to a storage table.
pub trait StorageSchema {
    type Item: Migration;

    /// The name of the table that the serializer is used for.
    fn table_name(&self) -> &'static str;
    /// The version of the serializer.
    fn version(&self) -> u64;

    /// Get the key of the given item.
    fn get_key(&self) -> Result<Vec<u8>>;

    /// Serialize the given item to a StorageEntry.
    fn to_storage_entry(&self) -> Result<StorageEntry>;
    /// Deserialize the given StorageEntry to an item.
    fn from_storage_entry(entry: &StorageEntry) -> Result<Self::Item>;
}
