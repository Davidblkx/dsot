use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct ReleaseMediaV0 {
    pub id: Uuid,
    pub release_id: Uuid,
    pub mbid: Option<Uuid>,
    pub position: u32,
    pub format: u32,
    pub count: u32,
}

pub enum ReleaseMediaFormat {
    Unknown,
    CD,
    Vinyl,
    Digital,
    Cassette,
}

crate::dsot_storage_declare_model!(
    ReleaseMedia {
        0: ReleaseMediaV0
    }
    "
    Documentation here
    "
);
