use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Album {
    pub id: uuid::Uuid,
    pub name: String,
    pub aliases: Vec<String>,
    pub mbid: Option<uuid::Uuid>,
}
