use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Tokio error: {0}")]
    TokioError(#[from] tokio::task::JoinError),
    #[error("Runtime error: {0}")]
    RuntimeError(#[from] dsot_runtime::error::RuntimeError),
}

pub type ServerResult<T> = Result<T, ServerError>;

pub struct HttpError {
    pub status_code: axum::http::StatusCode,
    pub message: String,
}

impl axum::response::IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        (self.status_code, self.message).into_response()
    }
}

pub type HttpResult<T> = Result<axum::Json<T>, HttpError>;

impl HttpError {
    pub fn handle_runtime<T>(value: dsot_runtime::error::Result<T>) -> HttpResult<T> {
        match value {
            Ok(v) => Ok(axum::Json(v)),
            Err(e) => Err(HttpError {
                status_code: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                message: e.to_string(),
            }),
        }
    }
}
