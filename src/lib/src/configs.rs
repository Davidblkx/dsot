use std::path::PathBuf;

use dsot_config::{ConfigOptions, DsotConfig, DsotConfigError};

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

pub fn load_config(
    config_file: &Option<String>,
) -> Result<DsotConfig<ConfigValue>, DsotConfigError> {
    let mut options = ConfigOptions::new()
        .auto_detect()
        .create_if_missing()
        .use_env();

    if let Some(file) = config_file {
        options = options.with_config_path(file.clone());
    }

    let config: DsotConfig<ConfigValue> = DsotConfig::load(options, ConfigValue::default())?;

    Ok(config)
}

pub fn load_mobile_config() -> Result<DsotConfig<ConfigValue>, DsotConfigError> {
    let options = ConfigOptions::new().from_data_dir().create_if_missing();

    let config: DsotConfig<ConfigValue> = DsotConfig::load(options, ConfigValue::default())?;

    Ok(config)
}
