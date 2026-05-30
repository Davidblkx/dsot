use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseManagerError {
    #[error("Path is not a valid directory")]
    PathIsNotAFolder,
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("SQLite error: {0}")]
    SQLiteDatabaseError(#[from] sqlx::Error),
    #[error("Redb error: {0}")]
    RedbDatabaseError(#[from] redb::DatabaseError),
    #[error("Migration error: {0}")]
    MigrationError(#[from] sqlx::migrate::MigrateError),
}

pub type Result<T> = std::result::Result<T, DatabaseManagerError>;
