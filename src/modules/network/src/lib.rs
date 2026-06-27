mod address_book;
mod config;
mod error;
mod init;
mod machine_info;
mod node;
mod protocols;

pub use address_book::*;
pub use config::*;
pub use error::*;
pub use machine_info::*;
pub use node::*;

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NetworkCapability {
    pub db_sync: bool,
}

#[derive(Debug, Clone)]
pub struct DsotNetwork {
    pub endpoint: iroh::Endpoint,
    pub router: iroh::protocol::Router,
    pub address_book: AddressBook,
    pub capabilities: NetworkCapability,
}
