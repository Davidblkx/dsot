/// The StorageEntry struct is used to represent a single entry in the storage.
pub struct StorageEntry {
    /// The version of the entry. Used to determine how to deserialize the entry.
    pub version: u64,
    /// The public key of the entry.
    pub key: Vec<u8>,
    /// The serialized value of the entry.
    pub value: Vec<u8>,
    /// The table name that the entry belongs to.
    pub table: &'static str,
}
