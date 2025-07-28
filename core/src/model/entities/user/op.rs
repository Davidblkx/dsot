#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum UserUpdateOpV0 {
    SetName(String),
}

crate::dsot_storage_declare_model!(
UserUpdateOp {
    0: UserUpdateOpV0
} "Represents an operation to update a user in the database."
);
