use std::path::PathBuf;

use dsot_config::{ConfigOptions, DsotConfig};

use super::init::DsotCoreInitOptions;
use crate::{error::Result, network::NetworkConfig};

pub type DsotAppConfig = DsotConfig<ConfigValue>;

/// Known configuration for a DSOT application
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ConfigValue {
    /// Name of the user to load at startup
    pub user: String,
    /// Token for current node, used to validate external requests
    pub token: String,
    /// Minimun log level to use, possible values are trace, debug, info, warn, error
    pub log_level: Option<String>,
    /// Path to file to write logs
    pub log_file: Option<PathBuf>,
    /// Enable/disable network access
    pub use_network: bool,
    /// Network configuration
    pub network_config: NetworkConfig,
    /// Enable/disable remote control API. Only enabled with `use_network` is also enabled.
    pub use_server: bool,
    /// Enable/disable database synchronization with other devices
    pub use_db_sync: bool,
}

impl Default for ConfigValue {
    fn default() -> Self {
        Self {
            user: "main".into(),
            token: uuid::Uuid::now_v7().to_string(),
            log_level: None,
            log_file: None,
            use_network: true,
            network_config: NetworkConfig::default(),
            use_server: true,
            use_db_sync: true,
        }
    }
}

impl DsotCoreInitOptions {
    /// Loads the configuration from disk
    ///
    /// If system have full disk access, will lookup for
    /// config file in both current directory and root
    ///
    /// The file is created if not found
    pub fn load_config(&self) -> Result<DsotAppConfig> {
        let mut options = if self.cap.can_full_disk_access() {
            ConfigOptions::new()
                .auto_detect()
                .create_if_missing()
                .use_env()
        } else {
            ConfigOptions::new().from_data_dir().create_if_missing()
        };

        if let Some(file) = &self.config_file {
            options = options.with_config_path(file.to_owned());
        }

        let config: DsotConfig<ConfigValue> = DsotConfig::load(options, ConfigValue::default())?;

        Ok(config)
    }
}
