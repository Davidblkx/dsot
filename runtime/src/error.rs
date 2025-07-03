use bakunin_config::errors::ConfigError;
use dsot_core::error::DsotError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, RuntimeError>;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Internal error: {0}")]
    InternalError(#[from] DsotError),
    #[error("Error loading configuration: {0}")]
    ErrorLoadingConfig(#[from] ConfigError),
    #[error("Unknown error: {0}")]
    UnknownError(String),
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Database error: {0}")]
    DatabaseError(#[from] crate::infra::db::DbError),
    #[error("Migration error: {0}")]
    MigrationError(#[from] sqlx::migrate::MigrateError),
}
