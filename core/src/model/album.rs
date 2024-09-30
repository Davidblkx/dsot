use serde::{Deserialize, Serialize};

use crate::storage::TableSchema;
use crate::error::{DsotError, Result};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Album {
    pub id: uuid::Uuid,
    pub name: String,
    pub aliases: Vec<String>,
    pub mbid: Option<uuid::Uuid>,
}

pub struct AlbumSchema;

impl TableSchema for AlbumSchema {
    type Value = Album;

    fn table_name() -> &'static str {
        "album"
    }

    fn version() -> u64 {
        1
    }

    fn get_key<'a>(value: &'a Self::Value) -> &'a [u8] {
        value.id.as_bytes()
    }

    fn deserialize<'a>(version: u64, value: &'a [u8]) -> Result<Self::Value> {
        if version == 1 {
            let v: Album = bincode1::deserialize(&value)
                .map_err(|e| DsotError::DeserializationError(e))?;
            return Ok(v);
        }

        Err(DsotError::DataVersionMismatch)
    }

    fn serialize(value: &Self::Value) -> Result<Vec<u8>> {
        bincode1::serialize(value)
            .map_err(|e| DsotError::SerializationError(e))
    }

    fn update_version<'a>(version: u64, _: &'a [u8]) -> Result<Option<Vec<u8>>> {
        if version == 1 {
            return Ok(None);
        }

        Err(DsotError::DataVersionMismatch)
    }
}
