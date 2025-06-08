use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum InboxUpdateOpV0 {
    SetMbid(Option<Uuid>),
}

crate::dsot_storage_declare_model!(
InboxUpdateOp {
    0: InboxUpdateOpV0
} "Represents an operation to update a inbox in the database."
);