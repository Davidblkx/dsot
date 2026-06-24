use thiserror::Error;

#[derive(Debug, Error)]
pub enum DsotSerdeError {
    #[error("Error serializing to binary: {0}")]
    BinarySerializeError(#[from] rmp_serde::encode::Error),
    #[error("Error deserializing from binary: {0}")]
    BinaryDeserializeError(#[from] rmp_serde::decode::Error),
}

pub type Result<T> = std::result::Result<T, DsotSerdeError>;
