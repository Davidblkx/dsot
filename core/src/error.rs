use thiserror::Error;

#[derive(Error, Debug)]
pub enum DsotError {
    #[error("Data is for a different version of the application")]
    DataVersionMismatch,

    #[error("Error while deserializing data: {0}")]
    DeserializationError(bincode1::Error),

    #[error("Error while serializing data: {0}")]
    SerializationError(bincode1::Error),

    #[error("Error opening database: {0} - {1}")]
    OpenDatabaseError(String, String),

    #[error("Data format out of bounds error: {0}")]
    DataFormatError(String),

    #[error("Error handling bucket: {bucket} - {operation}: {error}")]
    TransactionError {
        bucket: String,
        operation: &'static str,
        error: String,
    },

    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Invalid UUID length")]
    InvalidUuidLength,

    #[error("Invalid OS UTF8 string")]
    InvalidOSString,

    #[error("Invalid data folder: {0}")]
    InvalidDataFolder(String),

    #[error("Unsupported client version: {0}")]
    UnsupportedClientVersion(u8),

    #[error("Unknown DB entity index: {0}")]
    UnknownDbEntity(u32),

    #[error("Database is closed")]
    SqlClosedConnection,

    #[error("SQL Error: {0}")]
    SqlError(#[from] sqlx::Error),

    #[error("SQL Error: [Missing relation] {0}")]
    SqlMissingRelation(String),

    #[error("SQL Migration Error: {0}")]
    SqlMigrationError(#[from] sqlx::migrate::MigrateError),

    #[error("Unknown DSOT error")]
    Unknown,
}

impl DsotError {
    pub fn to_err<T>(self) -> Result<T> {
        Err(self)
    }
}

pub type Result<T> = std::result::Result<T, DsotError>;
