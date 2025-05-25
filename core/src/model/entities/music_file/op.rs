use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum MusicFileUpdateOpV0 {
    SetPath(String),
    SetStorageId(Uuid),
    SetRecordingId(Option<Uuid>),
    SetSize(u32),
    SetFormat(u32),
    SetNeedBetter(bool),
    SetChromaprint(Option<String>)
}

crate::dsot_storage_declare_model!(
MusicFileUpdateOp {
    0: MusicFileUpdateOpV0
} "Represents an operation to update a music_file in the database."
);
