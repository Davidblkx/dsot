use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TrackUpdateOpV0 {
    SetReleaseMediaId(Uuid),
    SetMediaIndex(u32),
    SetReleaseIndex(u32),
    SetTrackNumber(u32),
    SetPosition(Option<String>),
    SetTitle(String),
    SetMbid(Option<Uuid>),
    SetRecordingId(Uuid),
}

crate::dsot_storage_declare_model!(
TrackUpdateOp {
    0: TrackUpdateOpV0
} "Represents an operation to update a track in the database."
);
