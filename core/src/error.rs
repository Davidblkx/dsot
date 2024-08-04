use thiserror::Error;

#[derive(Error, Debug)]
pub enum DsotError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] redb::DatabaseError),
    #[error("Failed to lock database")]
    DatabaseLockError,


    #[error("Unknown DSOT error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, DsotError>;