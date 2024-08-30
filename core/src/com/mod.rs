use serde::{Deserialize, Serialize};

mod request;
mod response;

pub use request::*;
pub use response::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ClientType {
    Embedded,
    HTTP,
    WebSocket,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TargetLibrary {
    Custom(String),
    Default,
}
