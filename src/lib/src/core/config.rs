use std::path::PathBuf;

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
