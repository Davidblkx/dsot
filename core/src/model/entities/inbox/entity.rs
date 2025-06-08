use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct InboxV0 {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub file: Option<String>,
    pub extra_info: Option<String>,
}

crate::dsot_storage_declare_model!(
    Inbox {
        0: InboxV0
    }
    "
    Entity for the inbox, which stores information about items that users want to keep track of.
    "
);
