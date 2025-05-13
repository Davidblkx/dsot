use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ReleaseUpdateOpV0 {
    SetMbid(Option<Uuid>),
    SetTitle(String),
    SetYear(i32),
    SetStatus(Option<u32>),
    SetCountry(Option<String>),
    SetDuration(Option<i64>),
    SetFormat(Option<String>),
    SetAlbumId(Uuid),
}

crate::dsot_storage_declare_model!(
    ReleaseUpdateOp {
        0: ReleaseUpdateOpV0
    } "Represents an operation to update a release in the database."
);
