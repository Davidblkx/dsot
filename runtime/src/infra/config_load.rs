//! Logic to search and load configuration files.

use bakunin_config::{
    ConfigBuilder, Priority, Value,
    builder::{ConfigFile, ConfigFileFinder, OSFolder},
    value_map,
};
use crate::error::Result;

/// Options for loading the configuration.
#[derive(Debug, Clone)]
pub struct ConfigLoader {
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
}

impl ConfigLoader {
    /// Creates a new `ConfigLoader` with default values.
    pub fn new() -> Self {
        ConfigLoader::default()
    }

    /// Whether to create a configuration file if it does not exist.
    pub fn with_create(mut self, create: bool) -> Self {
        self.create = create;
        self
    }

    /// Whether to search for configuration files in expected locations.
    pub fn with_search(mut self, search: bool) -> Self {
        self.search = search;
        self
    }

    /// Whether to use the environment variables to override the configuration.
    pub fn with_use_env(mut self, use_env: bool) -> Self {
        self.use_env = use_env;
        self
    }

    /// Sets the path to the configuration file.
    pub fn with_config_path(mut self, config_path: String) -> Self {
        self.config_path = Some(config_path);
        self
    }

    /// Loads the configuration based on the options specified in `ConfigLoader`.
    pub fn load_config(&self) -> Result<Value> {
        log::trace!("Loading configuration with options: {:?}", self);
        let value = self.create_config_builder()?.build();
        log::debug!("Configuration loaded successfully");
        Ok(value)
    }

    /// Creates a new `ConfigBuilder` with default values and options specified in `ConfigLoader`.
    pub fn create_config_builder(&self) -> Result<ConfigBuilder> {
        let default_value = value_map! {
            user: "root",
            logger: value_map! {
                enabled: false,
                level: "trace",
                use_file: false,
                use_console: true,
                to_folder: "./logs",
            }
        };

        log::trace!("Creating configuration builder with default values");
        let mut builder = ConfigBuilder::from_base(default_value.clone())?;

        if self.search {
            self.search_and_add_config_files(&mut builder, &default_value)?;
        }

        if let Some(config_path) = &self.config_path {
            log::debug!("Adding configuration file from path: {}", config_path);
            let file = if self.create {
                ConfigFile::new(config_path.into()).with_init(default_value.clone())
            } else {
                ConfigFile::new(config_path.into())
            };
            builder.add_config_file(file, Priority::FirstAvailable)?;
        }

        if self.use_env {
            log::debug!("Using environment variables to override configuration");
            builder.environment("DSOT_")?;
        }

        Ok(builder)
    }

    fn search_and_add_config_files(&self, builder: &mut ConfigBuilder, default_value: &Value) -> Result<()> {
        log::trace!("Searching for configuration files");
        let file_finder = ConfigFileFinder::for_file("dsot_config".into())
            .with_os_folder(OSFolder::UserHome)
            .with_os_folder(OSFolder::WorkingDirectory)
            .with_supported_extensions();

        let files = file_finder.find_all();
        if files.is_empty() && self.create {
            log::debug!("Configuration file not found, creating a new one");
            let root_path = file_finder
                .find_or_first()
                .expect("Failed to find configuration file");

            log::debug!("Creating configuration file at: {}", root_path.display());
            let file = ConfigFile::new(root_path).with_init(default_value.clone());
            builder
                .add_config_file(file, Priority::FirstAvailable)?;
        } else {
            for file in files {
                log::debug!("Adding configuration file: {}", file.display());
                let file = ConfigFile::new(file);
                builder
                    .add_config_file(file, Priority::FirstAvailable)?;
            }
        }

        Ok(())
    }
}

impl Default for ConfigLoader {
    /// Creates a new `ConfigLoader` with default options.
    fn default() -> Self {
        ConfigLoader {
            create: true,
            search: true,
            use_env: true,
            config_path: None,
        }
    }
}
