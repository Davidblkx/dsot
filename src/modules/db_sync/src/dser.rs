use serde::de::DeserializeOwned;

pub struct EntityMessagePack;

pub type Result<T> = ::core::result::Result<T, crate::DBSyncError>;

impl EntityMessagePack {
    pub fn deserialize<T: DeserializeOwned>(data: &[u8]) -> Result<T> {
        let value = rmp_serde::from_slice(data)?;
        Ok(value)
    }

    pub fn serialize<T: serde::Serialize>(data: T) -> Result<Vec<u8>> {
        let value = rmp_serde::to_vec(&data)?;
        Ok(value)
    }
}
