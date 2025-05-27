//! This module contains the infrastructure for the runtime.
//! So any logic that is not specific to the runtime itself, but rather to the environment it runs in,
//! such as default configuration, logging, or database connections, is placed here.

pub static DEFAULT_USER: &'static str = "root";

/// Configuration for the runtime environment.
pub struct Config {
    /// The location where persistent data is stored (e.g., database files).
    pub data_location: String,
    /// The user for the runtime, defaults to "root".
    pub user: String,
    /// The password for the runtime user, if any.
    pub password: Option<String>,
}

impl Config {
    /// Creates a new `Config` instance from a `bakunin_config::Value`.
    pub fn from_value(v: bakunin_config::Value) -> Self {
        Self {
            data_location: v.get("data_location").try_into_string().unwrap_or("./data".to_string()),
            user: v.get("user").try_into_string().unwrap_or(DEFAULT_USER.to_string()),
            password: v.get("password").try_into_string().ok()
        }
    }
}
