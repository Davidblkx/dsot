use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Tokio error: {0}")]
    TokioError(#[from] tokio::task::JoinError),
}

pub type ServerResult<T> = Result<T, ServerError>;
