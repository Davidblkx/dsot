use uuid::Uuid;

use crate::db::sql::SqlEntity;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ArtistV0 {
    pub id: Uuid,
    pub name: String,
}

crate::dsot_storage_declare_model!(Artist {
    0: ArtistV0
});

crate::dsot_storage_use_id_uuid!(Artist, "artist");

impl SqlEntity for Artist {
    fn table_name() -> &'static str {
        "artists"
    }

    fn columns() -> Vec<&'static str> {
        vec!["id", "name"]
    }

    fn values(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            self.name.to_string(),
        ]
    }
}
