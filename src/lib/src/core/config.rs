use std::path::PathBuf;

use dsot_config::{ConfigOptions, DsotConfig};

use super::init::DsotCoreInitOptions;
use crate::error::Result;

pub type DsotAppConfig = DsotConfig<ConfigValue>;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ConfigValue {
    pub user: String,
    pub token: String,
    pub log_level: Option<String>,
    pub log_file: Option<PathBuf>,
    pub use_network: bool,
    pub network_config: dsot_network::NetworkConfig,
}

impl Default for ConfigValue {
    fn default() -> Self {
        Self {
            user: "main".into(),
            token: uuid::Uuid::now_v7().to_string(),
            log_level: None,
            log_file: None,
            use_network: true,
            network_config: dsot_network::NetworkConfig::default(),
        }
    }
}

impl DsotCoreInitOptions {
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

        if self.cap.can_full_disk_access() {
        } else {
            options = options.from_data_dir();
        }

        let config: DsotConfig<ConfigValue> = DsotConfig::load(options, ConfigValue::default())?;

        Ok(config)
    }
}
