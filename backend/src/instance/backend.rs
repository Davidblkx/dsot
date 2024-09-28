use std::sync::Arc;

use super::BackendConfig;

pub struct DsotBackendInstance {
    pub default_library: Arc<String>,
}

impl DsotBackendInstance {
    pub fn new(config: &BackendConfig) -> Self {
        DsotBackendInstance {
            default_library: Arc::new(config.default_library.clone()),
        }
    }
}
