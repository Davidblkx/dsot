use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum RecordingUpdateOpV0 {
    SetMbid(Option<Uuid>),
    SetTitle(String),
    SetLength(Option<u32>),
    SetIsrc(Option<String>),
    SetWorkId(Option<Uuid>),
    SetYear(Option<i32>),
    SetDisambiguation(Option<String>),
}

crate::dsot_storage_declare_model!(
RecordingUpdateOp {
    0: RecordingUpdateOpV0
} "Represents an operation to update a recording in the database."
);
