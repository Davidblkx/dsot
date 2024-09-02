use thiserror::Error;

#[derive(Error, Debug)]
pub enum MusicBrainzError {
    #[error("Failed parsing URL: {0}")]
    InvalidURL(#[from] url::ParseError),

    #[error("Request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Failed to parse JSON: {0}")]
    JSONParseError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, MusicBrainzError>;
