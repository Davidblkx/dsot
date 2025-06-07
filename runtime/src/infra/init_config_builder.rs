use crate::error::{Result, RuntimeError};
use bakunin_config::{
    ConfigBuilder, Priority, Value,
    builder::{ConfigFile, ConfigFileFinder, OSFolder},
    value_map,
};
use log::{debug, trace};

/// Creates a new `ConfigBuilder` for initializing the runtime configuration.
///
/// It looks for dst_config.toml in:
///   - user's home directory
///   - current working directory
///   - environment variable `DSOT_<key>`
pub fn init_config_builder(create_root: bool) -> Result<ConfigBuilder> {
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

    let file_finder = ConfigFileFinder::for_file("dsot_config".into())
        .with_os_folder(OSFolder::UserHome)
        .with_os_folder(OSFolder::WorkingDirectory)
        .with_os_folder(OSFolder::AppConfig("dsot".into()))
        .with_supported_extensions();

    trace!("Searching for configuration files");
    let mut builder = ConfigBuilder::from_base(default_value.clone())
        .map_err(|e| RuntimeError::ErrorLoadingConfig(e))?;

    let files = file_finder.find_all();

    if files.is_empty() && create_root {
        debug!("Configuration file not found, using default values");
        let root_path = file_finder
            .find_or_first()
            .expect("Failed to find configuration file");

        debug!("Creating configuration file at: {}", root_path.display());
        let file = ConfigFile::new(root_path).with_init(default_value.clone());
        builder
            .add_config_file(file, Priority::FirstAvailable)
            .map_err(|e| RuntimeError::ErrorLoadingConfig(e))?;
    } else {
        for file in files {
            let file = ConfigFile::new(file);
            builder
                .add_config_file(file, Priority::FirstAvailable)
                .map_err(|e| RuntimeError::ErrorLoadingConfig(e))?;
        }
    }

    builder
        .environment("DSOT_")
        .map_err(|e| RuntimeError::ErrorLoadingConfig(e))?;

    Ok(builder)
}
