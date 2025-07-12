//! This module contains the infrastructure for the runtime.
//! So any logic that is not specific to the runtime itself, but rather to the environment it runs in,
//! such as default configuration, logging, or database connections, is placed here.
mod init_folder;
mod init_logger;

pub mod config;
pub mod db;

pub use config::Config;
pub use init_folder::*;
pub use init_logger::*;
