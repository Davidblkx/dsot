use std::{error, io};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Disconnected from data repository: {0}")]
    Disconnect(#[from] io::Error),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Operation not supported: {0}")]
    OperationNotSupported(String),
    #[error("Unexpected error: {0}")]
    UnexpectedError(#[from] Box<dyn error::Error>),
    #[error("Not found {entity:?} with id: {id:?}")]
    EntityNotFound {
        entity: &'static str,
        id: uuid::Uuid,
    },
}

pub type Result<T> = std::result::Result<T, RepositoryError>;
