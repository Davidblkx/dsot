use std::path::PathBuf;
use native_model::native_model;
use serde::{Deserialize, Serialize};

use crate::db::EntityKey;

pub const SONG_FILE_TABLE_NAME: &str = "song_file";
// pub const SONG_FILE_ID: u16 = 101;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[native_model(id = 101, version = 1)]
pub struct SongFileV1 {
    pub id: uuid7::Uuid,
    pub path: PathBuf
}

impl SongFileV1 {
    pub fn new(id: uuid7::Uuid, path: PathBuf) -> Self {
        Self {
            id,
            path
        }
    }

    pub fn create(path: PathBuf) -> Self {
        Self::new(uuid7::uuid7(), path)
    }

    pub fn key(&self) -> EntityKey {
        EntityKey::for_uuid(SONG_FILE_TABLE_NAME, &self.id)
    }
}
