use bakunin_config::errors::ConfigError;
use dsot_core::error::DsotError;

pub type Result<T> = std::result::Result<T, RuntimeError>;

#[derive(Debug)]
pub enum RuntimeError {
    InternalError(DsotError),
    ErrorLoadingConfig(ConfigError),
    UnknownError(String),
    IOError(std::io::Error),
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::InternalError(e) => write!(f, "Internal error: {}", e),
            RuntimeError::ErrorLoadingConfig(e) => write!(f, "Error loading config: {}", e),
            RuntimeError::UnknownError(msg) => write!(f, "Unknown error: {}", msg),
            RuntimeError::IOError(e) => write!(f, "IO error: {}", e),
        }
    }
}
