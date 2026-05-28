use std::path::PathBuf;

use bakunin_config::{BakuninConfig, Value};

mod error;
mod loader;
mod options;

pub use error::{DsotConfigError, Result};
pub use options::ConfigOptions;

#[derive(Debug)]
pub struct DsotConfig {
    pub data_location: PathBuf,
    pub user: String,
    pub inner: Value,
    pub handler: BakuninConfig,
}
