use serde::de::DeserializeOwned;
use thiserror::Error;

pub struct EntityMessagePack;

impl EntityMessagePack {
    pub fn deserialize<T: DeserializeOwned>(data: &[u8]) -> Result<T, MessagePackError> {
        let value = rmp_serde::from_slice(data)?;
        Ok(value)
    }

    pub fn serialize<T: serde::Serialize>(data: T) -> Result<Vec<u8>, MessagePackError> {
        let value = rmp_serde::to_vec(&data)?;
        Ok(value)
    }
}

#[derive(Error, Debug)]
pub enum MessagePackError {
    #[error("error deserializing: {0}")]
    DeserializeError(#[from] rmp_serde::decode::Error),
    #[error("error serializing: {0}")]
    SerializeError(#[from] rmp_serde::encode::Error),
}
