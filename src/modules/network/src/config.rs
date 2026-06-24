use std::path::PathBuf;
use std::sync::Arc;

use dsot_db_sync::manager::DatabaseManagerProvider;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct NetworkConfig {
    pub persist_key: bool,
    pub use_db_sync: bool,
    pub key_file: Option<String>,
    pub address_book: Option<String>,
    pub public_name: Option<String>,
    pub public_desc: Option<String>,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            persist_key: true,
            use_db_sync: true,
            key_file: None,
            address_book: None,
            public_name: None,
            public_desc: None,
        }
    }
}

pub struct NetworkInitOptions {
    pub data_folder: PathBuf,
    pub config: NetworkConfig,
    pub manager: Arc<dyn DatabaseManagerProvider + Send + Sync>,
}

impl std::fmt::Debug for NetworkInitOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NetworkInitOptions")
            .field("data_folder", &self.data_folder)
            .field("config", &self.config)
            .finish()
    }
}
