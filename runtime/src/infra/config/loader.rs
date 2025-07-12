use super::Config;
use crate::error::Result;

use bakunin_config::file_finder::FileFinder;
use bakunin_config::{BakuninConfig, value_map};

static CONFIG_FILE: &'static str = ".dsot";

/// Options for loading the configuration.
#[derive(Debug, Clone)]
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
}

impl Default for ConfigOptions {
    /// Creates a new `ConfigLoader` with default options.
    fn default() -> Self {
        ConfigOptions {
            create: true,
            search: true,
            use_env: true,
            config_path: None,
        }
    }
}

impl Config {
    pub fn create(options: ConfigOptions) -> Result<Self> {
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

        let mut handler = BakuninConfig::new().with_memory_layer("default", default_value.clone());

        if options.search {
            let global_search = FileFinder::new(CONFIG_FILE)
                .with_supported_extensions()
                .with_user_home()
                .with_user_config()
                .find_first(true)?;

            handler.add_file_layer("global", global_search.path)?;

            let local_search = FileFinder::new(CONFIG_FILE)
                .with_supported_extensions()
                .with_working_directory()
                .find_first(true)?;

            if local_search.exists {
                handler.add_file_layer("local", local_search.path)?;
            }
        }

        if options.create {
            if let Some(cfg) = handler.get_layer("global") {
                if !cfg.has_value() {
                    cfg.write_value(&default_value)?;
                }
            }
        }

        if let Some(path) = options.config_path {
            handler.add_file_layer("custom", std::path::PathBuf::from(path))?;
        }

        if options.use_env {
            handler.add_environment_layer("env", "DSOT_");
        }

        Config::from_handler(handler)
    }
}
