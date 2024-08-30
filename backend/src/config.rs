use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct BackendConfig {
    pub data_path: PathBuf,
    pub default_library: String,
}
