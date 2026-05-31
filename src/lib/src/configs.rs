use dsot_config::{ConfigOptions, DsotConfig, DsotConfigError};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ConfigValue {
    user: String,
}

impl Default for ConfigValue {
    fn default() -> Self {
        Self {
            user: "main".into(),
        }
    }
}

pub fn load_config() -> Result<DsotConfig<ConfigValue>, DsotConfigError> {
    let options = ConfigOptions::new()
        .auto_detect()
        .create_if_missing()
        .use_env();

    let config: DsotConfig<ConfigValue> = DsotConfig::load(options, ConfigValue::default())?;

    Ok(config)
}
