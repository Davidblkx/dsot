use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct UserV0 {
    pub id: Uuid,
    pub name: String,
}

crate::dsot_storage_declare_model!(
    User {
        0: UserV0
    }
    "
    Documentation here
    "
);
