#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum StorageUpdateOpV0 {
    SetDescription(Option<String>),
    SetKind(String),
    SetPath(Option<String>),
    SetInfo(Option<String>)
}

crate::dsot_storage_declare_model!(
StorageUpdateOp {
    0: StorageUpdateOpV0
} "Represents an operation to update a storage in the database."
);
