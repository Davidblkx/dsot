use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct StorageV0 {
    pub id: Uuid,
    /// Description of the storage device
    pub description: String,
    /// The mount point of the storage device
    /// e.g. "/mnt/storage1"
    /// or "C:\\"
    pub mount: String,
    /// The root directory where media files are stored
    /// e.g. "media/music"
    pub root: String,
    /// The serial number of the storage device
    pub serial_number: String,
    /// If true, this storage is the default storage for media files
    pub is_default: bool
}

crate::dsot_storage_declare_model!(
    Storage {
        0: StorageV0
    }
    "
    Represents a storage device used for storing media files.
    "
);
