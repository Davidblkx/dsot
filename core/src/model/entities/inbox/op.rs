use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum InboxUpdateOpV0 {
    SetUserId(Uuid),
    SetTitle(Option<String>),
    SetArtist(Option<String>),
    SetAlbum(Option<String>),
    SetFile(Option<String>),
    SetExtraInfo(Option<String>),
}

crate::dsot_storage_declare_model!(
InboxUpdateOp {
    0: InboxUpdateOpV0
} "Represents an operation to update a inbox in the database."
);
