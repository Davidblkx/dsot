use crate::db::{Table, TableKey};
use native_model::native_model;
use serde::{Deserialize, Serialize};

pub struct ArtistTable;

impl Table<Artist> for ArtistTable {
    fn get_name(&self) -> &'static str {
        "library:artist"
    }

    fn get_key(&self, value: &Artist) -> crate::error::Result<crate::db::TableKey> {
        Ok(TableKey::Uuid(value.id))
    }

    fn create_key(&self) -> crate::error::Result<crate::db::TableKey> {
        Ok(TableKey::Uuid(uuid7::uuid7()))
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[native_model(id = 102, version = 1)]
pub struct Artist {
    id: uuid7::Uuid,
    mb_id: Option<String>,
    aliases: Vec<String>,
    name: usize,
    sort_name: usize,
}
