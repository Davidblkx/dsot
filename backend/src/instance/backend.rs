use std::sync::Arc;

use dsot_core::db::LocalDBManager;

use super::BackendConfig;

pub struct DsotBackendInstance {
    pub db_manager: LocalDBManager,
    pub default_library: Arc<String>,
}

impl DsotBackendInstance {
    pub fn new(config: &BackendConfig) -> Self {
        let db_manager = LocalDBManager::new(config.data_path.as_path());
        DsotBackendInstance {
            db_manager,
            default_library: Arc::new(config.default_library.clone()),
        }
    }
}
