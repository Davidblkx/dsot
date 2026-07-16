#[derive(Debug, thiserror::Error)]
pub enum DsotError {
    #[error("Configuration error: {0}")]
    ConfigError(#[from] dsot_config::DsotConfigError),
    #[error("Log init error: {0}")]
    LogInitError(#[from] fern::InitError),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Network: disconnected")]
    NetworkDisconnected,
    #[error("Network: attempted to connect to an already open connection")]
    NetworkDoubleConnection,
    #[error("Network: Iroh bind error: {0}")]
    IrohBind(#[from] iroh::endpoint::BindError),
    #[error("Network: Iroh accept error: {0}")]
    IrohAcceptError(#[from] iroh::protocol::AcceptError),
    #[error("Network: Iroh connect error: {0}")]
    IrohConnectError(#[from] iroh::endpoint::ConnectError),
    #[error("Network: Iroh generic error: {0}")]
    IrohError(String),
    #[error("Serialize/Deserialize error: {0}")]
    SerDeError(#[from] dsot_serde::DsotSerdeError),
    #[error("Network: Device communication error: {0}")]
    NetworkDeviceError(String),
}

pub type Result<T> = std::result::Result<T, DsotError>;
