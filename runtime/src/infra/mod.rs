//! This module contains the infrastructure for the runtime.
//! So any logic that is not specific to the runtime itself, but rather to the environment it runs in,
//! such as default configuration, logging, or database connections, is placed here.
mod init_logger;
mod init_config_builder;

pub mod config;

pub use config::Config;
pub use init_logger::*;
pub use init_config_builder::*;
