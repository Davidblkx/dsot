use crate::{storage_schema, storage_schema_v0};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileStorageV0 {
    pub id: uuid::Uuid,
    pub path: std::path::PathBuf,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileStorage {
    pub id: uuid::Uuid,
    pub path: std::path::PathBuf,
    pub disk_id: Option<String>
}

storage_schema_v0!(FileStorageV0["file_storage"] => |v: &FileStorageV0| v.id.as_bytes().to_vec());

// TODO: Implement macro as a list of versions and mappings+
storage_schema!{
    1 => {
        FileStorage["file_storage"] => |v: &FileStorage| v.id.as_bytes().to_vec(),
        FileStorageV0 => |v: &FileStorageV0| FileStorage {
                            id: v.id.clone(),
                            path: v.path.clone(),
                            disk_id: None
                        }
    }
}
