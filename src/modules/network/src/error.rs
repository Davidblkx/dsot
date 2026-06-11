use thiserror::Error;

#[derive(Debug, Error)]
pub enum DsotNetworkError {
    #[error("Unknown network error")]
    Unknown,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Iroh bind error: {0}")]
    IrohBind(#[from] iroh::endpoint::BindError),
}

pub type Result<T> = std::result::Result<T, DsotNetworkError>;
