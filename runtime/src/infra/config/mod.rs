//! Module for configuration management.
//! This module provides functionality to load and manage configuration settings for the runtime environment.

pub mod dirs;
pub mod loader;
pub mod logger;
pub mod read_value;

use crate::error::Result;
use bakunin_config::{BakuninConfig, Value};
use std::path::PathBuf;

pub use loader::{
    CONFIG_FILE_NAME, CUSTOM_FILE_LAYER_NAME, ConfigOptions, GLOBAL_FILE_LAYER_NAME,
    LOCAL_FILE_LAYER_NAME,
};
pub use logger::*;

static DEFAULT_USER: &'static str = "root";

/// Configuration for the runtime environment.
pub struct Config {
    /// The location where persistent data is stored (e.g., database files).
    pub data_location: PathBuf,
    /// The user for the runtime, defaults to "root".
    pub user: String,
    /// Configuration for logging in the runtime environment.
    pub logger: Option<LogConfig>,
    /// Raw configuration value, if available.
    pub raw: Value,
    /// Configuration handler for the runtime.
    pub handler: BakuninConfig,
}

impl Config {
    pub fn from_handler(handler: BakuninConfig) -> Result<Self> {
        let v = handler.build_value(true)?;
        let data_location = dirs::get_data_location(&v);
        let logger = {
            let log_cfg = v.get("logger");
            if log_cfg.is_map() {
                Some(LogConfig::from_value(log_cfg, &data_location))
            } else {
                None
            }
        };

        Ok(Config {
            data_location,
            user: v.get("user").into_string_or(DEFAULT_USER.to_string()),
            logger,
            raw: v,
            handler,
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            data_location: dirs::get_data_location(&Value::default()),
            user: DEFAULT_USER.to_string(),
            logger: None,
            raw: Value::None,
            handler: BakuninConfig::new(),
        }
    }
}
