use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ArtistUpdateOpV0 {
    SetMbid(Option<Uuid>),
    SetName(String),
    SetSortName(Option<String>),
    SetArtistTypeId(u32),
}

crate::dsot_storage_declare_model!(
    ArtistUpdateOp {
        0: ArtistUpdateOpV0
    }
    "Represents an operation to update an artist in the database."
);
