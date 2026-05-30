use bakunin_config::BakuninError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DsotConfigError {
    #[error("Config Error: {0}")]
    BakuninError(#[from] BakuninError),
}

pub type Result<T> = std::result::Result<T, DsotConfigError>;
