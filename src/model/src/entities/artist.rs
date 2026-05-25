use dsot_derive::SyncEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, Default, SyncEntity)]
#[table(artists)]
pub struct Artist {
    pub id: Uuid,
    pub name: String,
    pub sort_name: Option<String>,
    pub aliases: sqlx::types::Json<Vec<String>>,
}

impl Artist {
    pub fn new(id: Uuid, name: String) -> Self {
        Self {
            id,
            name,
            sort_name: None,
            aliases: sqlx::types::Json(vec![]),
        }
    }

    pub fn with_aliases(mut self, aliases: Vec<String>) -> Self {
        self.aliases.0 = aliases;
        self
    }

    pub fn add_aliase(&mut self, alias: String) {
        self.aliases.0.push(alias);
    }

    pub fn set_aliases(&mut self, aliases: Vec<String>) {
        self.aliases.0 = aliases;
    }
}
