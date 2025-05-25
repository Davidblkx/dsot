#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum StorageUpdateOpV0 {
    SetDescription(String),
    SetMount(String),
    SetRoot(String),
    SetSerialNumber(String),
    SetIsDefault(bool),
}

crate::dsot_storage_declare_model!(
StorageUpdateOp {
    0: StorageUpdateOpV0
} "Represents an operation to update a storage in the database."
);
