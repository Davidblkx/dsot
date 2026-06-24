use crate::error::Result;

pub struct BinarySerde;

impl BinarySerde {
    pub fn serialize<T>(value: &T) -> Result<Vec<u8>>
    where
        T: serde::Serialize,
    {
        let value = rmp_serde::to_vec(value)?;
        Ok(value)
    }

    pub fn deserialize<T>(data: &[u8]) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let value = rmp_serde::from_slice(data)?;
        Ok(value)
    }
}

#[macro_export]
macro_rules! serde_binary {
    ($id:ident) => {
        impl $id {
            pub fn to_binary(&self) -> Result<Vec<u8>> {
                let v = $crate::BinarySerde::serialize(self)?;
                Ok(v)
            }

            pub fn from_binary(data: &[u8]) -> Result<Self> {
                let v = $crate::BinarySerde::deserialize(data)?;
                Ok(v)
            }
        }
    };
}
