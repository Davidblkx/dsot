use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum AlbumUpdateOpV0 {
    SetMbid(Option<Uuid>),
    SetTitle(String),
    SetYear(Option<i16>),
}

crate::dsot_storage_declare_model!(
    AlbumUpdateOp {
        0: AlbumUpdateOpV0
    } "Represents an operation to update an album in the database."
);
