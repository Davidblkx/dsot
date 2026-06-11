use std::path::PathBuf;

use dsot_db_sync::{DatabaseManager, Result};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct NetworkConfig {
    pub persist_key: bool,
    pub use_db_sync: bool,
    pub key_file: Option<String>,
    pub address_book: Option<String>,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            persist_key: true,
            use_db_sync: true,
            key_file: None,
            address_book: None,
        }
    }
}

#[derive(Debug)]
pub struct NetworkInitOptions {
    pub data_folder: PathBuf,
    pub config: NetworkConfig,
    pub manager: fn(id: &str) -> Result<DatabaseManager>,
}
