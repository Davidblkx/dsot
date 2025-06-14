use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct StorageV0 {
    pub id: Uuid,
    /// Description of the storage device
    pub description: Option<String>,
    /// The root directory where media files are stored
    /// e.g. "media/music"
    pub path: Option<String>,
    /// The serial number of the storage device
    pub kind: String,
    /// Extra information about the storage device
    pub info: Option<String>
}

crate::dsot_storage_declare_model!(
    Storage {
        0: StorageV0
    }
    "
    Represents a storage device used for storing media files.
    "
);
