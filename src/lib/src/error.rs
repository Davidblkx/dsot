#[derive(Debug, thiserror::Error)]
pub enum DsotError {
    #[error("Configuration error: {0}")]
    ConfigError(#[from] dsot_config::DsotConfigError),
    #[error("Log init error: {0}")]
    LogInitError(#[from] fern::InitError),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, DsotError>;
