use thiserror::Error;

pub type Result<T> = std::result::Result<T, DsotDatabaseError>;

#[derive(Debug, Error)]
pub enum DsotDatabaseError {
    #[error("Redb storage failure: {0}")]
    RedbStorageError(#[from] redb::StorageError),
    #[error("Redb transaction failed or aborted: {0}")]
    RedbTransactionError(#[from] redb::TransactionError),
    #[error("Redb commit failed: {0}")]
    RedbCommitError(#[from] redb::CommitError),
    #[error("Redb table error: {0}")]
    RedbTableError(#[from] redb::TableError),
    #[error("Repository error: {0}")]
    RepositoryError(#[from] crate::repo::RepositoryError),
    #[error("SQLite error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("{0}")]
    SerializationError(#[from] crate::dser::MessagePackError),
    #[error("Cannot insert journal for {0} in table {1}")]
    TableMissmatchError(String, &'static str),
    #[error("Repository not available for table: {0}")]
    RepositoryNotFound(String),
}
