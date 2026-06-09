use thiserror::Error;

#[derive(Debug, Error)]
pub enum SyncDatabaseError {
    #[error("Database operation error: {0}")]
    DatabaseError(#[from] crate::database::DsotDatabaseError),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] crate::dser::MessagePackError),
}

pub type SyncResult<T> = Result<T, SyncDatabaseError>;
