#[derive(Debug, Clone, Default)]
pub struct ConfigOptions {
    /// Whether to create a configuration file if it does not exist.
    pub create: bool,
    /// Weather to search for configuration files in expected locations.
    pub search: bool,
    /// Whether to use the environment variables to override the configuration.
    /// It will always override the configuration values found in the configuration files.
    /// If `true`, it will use the environment variables prefixed with `DSOT_` to override the configuration values.
    pub use_env: bool,
    /// Path to the configuration file, if combined with `search` it will be last in priority,
    /// so it will override any found configuration values.
    pub config_path: Option<String>,
    /// Global config is read from data_dir (ignores search options)
    pub from_data_dir: bool,
}

impl ConfigOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_if_missing(mut self) -> Self {
        self.create = true;
        self
    }

    pub fn auto_detect(mut self) -> Self {
        self.search = true;
        self
    }

    pub fn use_env(mut self) -> Self {
        self.use_env = true;
        self
    }

    pub fn with_config_path(mut self, path: String) -> Self {
        self.config_path = Some(path);
        self
    }

    pub fn from_data_dir(mut self) -> Self {
        self.from_data_dir = true;
        self
    }
}
