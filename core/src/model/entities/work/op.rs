use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum WorkUpdateOpV0 {
    SetMbid(Option<Uuid>),
    SetTitle(String),
    SetKind(Option<String>),
    SetLanguage(Option<String>),
    SetDisambiguation(Option<String>),
}

crate::dsot_storage_declare_model!(
WorkUpdateOp {
    0: WorkUpdateOpV0
} "Represents an operation to update a work in the database."
);
