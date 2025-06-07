//! This module contains the infrastructure for the runtime.
//! So any logic that is not specific to the runtime itself, but rather to the environment it runs in,
//! such as default configuration, logging, or database connections, is placed here.
mod init_config_builder;
mod init_folder;
mod init_logger;

pub mod config;

pub use config::Config;
pub use init_config_builder::*;
pub use init_folder::*;
pub use init_logger::*;
