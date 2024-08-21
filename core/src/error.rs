use thiserror::Error;

#[derive(Error, Debug)]
pub enum DsotError {
    #[error("Failed to lock database")]
    DatabaseLockError,
    #[error("Database connection error: {0}")]
    DatabaseConnectionError(#[from] diesel::ConnectionError),
    #[error("Duplicated database initialization: {0}")]
    DatabaseDuplicatedInit(String),
    #[error("Database not initialized: {0}")]
    DatabaseNotFound(String),
    #[error("Database migration failed: {0}")]
    DatabaseMigrationError(String),
    #[error("Database query error: {0}")]
    DatabaseQueryError(#[from] diesel::result::Error),

    #[error("Failed to decode/encode native model: {0}")]
    NativeModelError(#[from] native_model::Error),

    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Invalid UUID length")]
    InvalidUuidLength,

    #[error("Invalid OS UTF8 string")]
    InvalidOSString,

    #[error("Invalid data folder: {0}")]
    InvalidDataFolder(String),

    #[error("Unknown DSOT error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, DsotError>;
