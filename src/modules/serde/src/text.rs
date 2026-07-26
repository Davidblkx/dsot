use std::path::PathBuf;

use crate::error::Result;

pub struct TextSerde;

impl TextSerde {
    pub fn serialize<T>(value: &T) -> Result<String>
    where
        T: serde::Serialize,
    {
        let result = toml::to_string(value)?;
        Ok(result)
    }

    pub fn deserialize<T>(data: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let result = toml::from_str(data)?;
        Ok(result)
    }

    pub fn write_file<T>(path: PathBuf, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        let content = toml::to_string_pretty(value)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn read_file<T>(path: PathBuf) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let content = std::fs::read(path)?;
        let result = toml::from_slice(content.as_slice())?;
        Ok(result)
    }
}

#[macro_export]
macro_rules! serde_text {
    ($id:ident) => {
        impl $id {
            pub fn to_text(&self) -> $crate::Result<String> {
                $crate::TextSerde::serialize(self)
            }

            pub fn from_text(data: &str) -> $crate::Result<Self> {
                $crate::TextSerde::deserialize(data)
            }

            pub fn to_file(&self, path: PathBuf) -> $crate::Result<()> {
                $crate::TextSerde::write_file(path, self)
            }

            pub fn from_file(path: PathBuf) -> $crate::Result<Self> {
                $crate::TextSerde::read_file(path)
            }
        }
    };
}
