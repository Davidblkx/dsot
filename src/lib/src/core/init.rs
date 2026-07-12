#[derive(Debug, Clone)]
pub struct DsotCoreInitOptions {
    pub debug: bool,
    pub config_file: Option<String>,
    pub is_mobile: bool,
}

impl DsotCoreInitOptions {
    pub fn new() -> Self {
        Self {
            debug: false,
            config_file: None,
            is_mobile: false,
        }
    }
}
