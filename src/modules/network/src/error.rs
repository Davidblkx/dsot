use thiserror::Error;

#[derive(Debug, Error)]
pub enum DsotNetworkError {
    #[error("Unknown network error")]
    Unknown,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Iroh bind error: {0}")]
    IrohBind(#[from] iroh::endpoint::BindError),
    #[error("Invalid address book format: {0}")]
    TomlDeError(#[from] toml::de::Error),
    #[error("Invalid address book structure: {0}")]
    TomlSerError(#[from] toml::ser::Error),
    #[error("Serialize/Deserialize error: {0}")]
    SerDeError(#[from] dsot_serde::DsotSerdeError),
    #[error("Iroh accept error: {0}")]
    IrohAcceptError(#[from] iroh::protocol::AcceptError),
    #[error("Iroh connect error: {0}")]
    IrohConnectError(#[from] iroh::endpoint::ConnectError),
    #[error("Empty message")]
    EmptyMessage,
}

pub type Result<T> = std::result::Result<T, DsotNetworkError>;
