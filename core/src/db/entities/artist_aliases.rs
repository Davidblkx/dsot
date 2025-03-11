use uuid::Uuid;

use crate::db::sql::{SqlEntity, SqlValue};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct ArtistAliaseV0 {
    pub id: Uuid,
    pub artist_id: Uuid,
    pub name: String,
}

crate::dsot_storage_declare_model!(ArtistAliase {
    0: ArtistAliaseV0
});

crate::dsot_storage_use_id_uuid!(ArtistAliase, "artist_aliases");

impl SqlEntity for ArtistAliase {
    fn table_name() -> &'static str {
        "artist_aliases"
    }

    fn columns() -> Vec<&'static str> {
        vec!["id", "artist_id", "name"]
    }

    fn values(&self) -> Vec<String> {
        vec![
            SqlValue::uuid(&self.id),
            SqlValue::uuid(&self.artist_id),
            SqlValue::string(&self.name),
        ]
    }
}

impl ArtistAliase {
    pub fn new(artist_id: &Uuid, name: &str) -> Self {
        Self {
            id: Uuid::now_v7(),
            artist_id: artist_id.clone(),
            name: name.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {

}
