use crate::error::Result;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseHandlerError {
    #[error("Journal entry already applied")]
    DuplicatedEntry,

    #[error("Database connection is not available")]
    DatabaseConnectionUnavailable,

    #[error("Operation not supported")]
    UnsupportedOperation,

    #[error("Path not available for in memory database")]
    PathNotAvailable,

    #[error("Invalid path: {0}")]
    InvalidPath(&'static str),
}

impl DatabaseHandlerError {
    pub fn to_err<T>(self) -> Result<T> {
        Err(self.into())
    }
}
