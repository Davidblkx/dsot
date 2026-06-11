mod config;
mod error;
mod init;

pub use config::*;
pub use error::*;

#[derive(Debug, Clone)]
pub struct DsotNetwork {
    pub endpoint: iroh::Endpoint,
    pub router: iroh::protocol::Router,
}
