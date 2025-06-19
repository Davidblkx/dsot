//! Module for configuration management.
//! This module provides functionality to load and manage configuration settings for the runtime environment.

pub mod dirs;
pub mod logger;

use bakunin_config::Value;
use std::path::PathBuf;

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
    pub raw: Option<Value>,
}

impl Config {
    /// Creates a new `Config` instance from a `bakunin_config::Value`.
    pub fn from_value(v: bakunin_config::Value) -> Self {
        let data_location = dirs::get_data_location(&v);
        let logger = {
            let log_cfg = v.get("logger");
            if log_cfg.is_map() {
                Some(LogConfig::from_value(log_cfg, &data_location))
            } else {
                None
            }
        };

        Self {
            data_location,
            user: v.get("user").into_string_or(DEFAULT_USER.to_string()),
            logger,
            raw: Some(v),
        }
    }
}
