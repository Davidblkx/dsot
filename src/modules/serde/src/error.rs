use thiserror::Error;

#[derive(Debug, Error)]
pub enum DsotSerdeError {
    #[error("Error serializing to binary: {0}")]
    BinarySerializeError(#[from] rmp_serde::encode::Error),
    #[error("Error deserializing from binary: {0}")]
    BinaryDeserializeError(#[from] rmp_serde::decode::Error),
    #[error("Error serializing to toml: {0}")]
    TextSerializeError(#[from] toml::ser::Error),
    #[error("Error deserializing from toml: {0}")]
    TextDeserializeError(#[from] toml::de::Error),
    #[error("Error handling file: {0}")]
    IOError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, DsotSerdeError>;
