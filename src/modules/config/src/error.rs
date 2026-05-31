use bakunin_config::{BakuninError, model::ModelError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DsotConfigError {
    #[error("Config Error: {0}")]
    BakuninError(#[from] BakuninError),
    #[error("Base type can't be serialized: {0}")]
    BakuninModelError(#[from] ModelError),
}

pub type Result<T> = std::result::Result<T, DsotConfigError>;
