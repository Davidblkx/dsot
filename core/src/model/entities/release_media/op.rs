use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ReleaseMediaUpdateOpV0 {
    SetReleaseId(Uuid),
    SetFormat(u32),
    SetCount(u32),
}

crate::dsot_storage_declare_model!(
ReleaseMediaUpdateOp {
    0: ReleaseMediaUpdateOpV0
} "Represents an operation to update a release_media in the database."
);
