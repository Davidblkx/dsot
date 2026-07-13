use super::cap::Capability;

#[derive(Debug, Clone)]
pub struct DsotCoreInitOptions {
    pub debug: bool,
    pub config_file: Option<String>,
    pub cap: Capability,
}

impl DsotCoreInitOptions {
    pub fn new() -> Self {
        Self {
            debug: false,
            config_file: None,
            cap: Capability::new(),
        }
    }
}

impl DsotCoreInitOptions {
    pub fn with_debug(mut self, value: bool) -> Self {
        self.debug = value;
        self
    }

    pub fn with_config_file(mut self, value: Option<String>) -> Self {
        self.config_file = value;
        self
    }

    pub fn with_cap(mut self, value: Capability) -> Self {
        self.cap = value;
        self
    }
}
