use thiserror::Error;

#[derive(Error, Debug)]
/// Error type for database operations in the runtime.
pub enum DbError {
    #[error("Failed to connect to the database: {0}")]
    ConnectionError(#[from] sqlx::Error),

    #[error("Backup already exists at: {0}")]
    BackupExists(String),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}
