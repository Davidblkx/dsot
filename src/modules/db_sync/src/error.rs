use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum DBSyncError {
    // redb storage / transaction / commit / table errors
    #[error("Redb storage failure: {0}")]
    RedbStorageError(#[from] redb::StorageError),
    #[error("Redb transaction failed or aborted: {0}")]
    RedbTransactionError(#[from] redb::TransactionError),
    #[error("Redb commit failed: {0}")]
    RedbCommitError(#[from] redb::CommitError),
    #[error("Redb table error: {0}")]
    RedbTableError(#[from] redb::TableError),
    #[error("Redb error: {0}")]
    RedbDatabaseError(#[from] redb::DatabaseError),

    // sqlx / SQLite errors
    #[error("SQLite error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    // serialization / messagepack errors
    #[error("error deserializing: {0}")]
    DeserializeError(#[from] rmp_serde::decode::Error),
    #[error("error serializing: {0}")]
    SerializeError(#[from] rmp_serde::encode::Error),

    // io errors
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    // migration errors
    #[error("Migration error: {0}")]
    MigrationError(#[from] sqlx::migrate::MigrateError),

    // entity / sync errors
    #[error("entity [{0}] not found: {1}")]
    EntityNotFound(&'static str, Uuid),
    #[error("Cannot insert journal for {0} in table {1}")]
    TableMissmatchError(String, &'static str),
    #[error("Repository not available for table: {0}")]
    RepositoryNotFound(String),
    #[error("Sync error: {0}")]
    SyncError(String),
    #[error("Path is not a valid directory")]
    PathIsNotAFolder,
}

pub type Result<T> = std::result::Result<T, DBSyncError>;
