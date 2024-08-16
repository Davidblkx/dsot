use thiserror::Error;

#[derive(Error, Debug)]
pub enum DsotError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] redb::DatabaseError),
    #[error("Database transaction error: {0}")]
    DatabaseTransactionError(#[from] redb::TransactionError),
    #[error("Database table error: {0}")]
    DatabaseTableError(#[from] redb::TableError),
    #[error("Database storage error: {0}")]
    DatabaseStorageError(#[from] redb::StorageError),
    #[error("Failed to lock database")]
    DatabaseLockError,
    #[error("Failed to commit transaction: {0}")]
    DatabaseCommitError(#[from] redb::CommitError),

    #[error("Failed to decode/encode native model: {0}")]
    NativeModelError(#[from] native_model::Error),

    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Invalid UUID length")]
    InvalidUuidLength,

    #[error("Unknown DSOT error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, DsotError>;
