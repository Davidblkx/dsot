use std::path::PathBuf;

pub struct LibConfig {
    pub name: String,
}

pub struct SetupConfig {
    pub local_data: PathBuf,
    pub lib: LibConfig,
}
