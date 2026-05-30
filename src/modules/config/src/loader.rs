use std::path::PathBuf;

use bakunin_config::{BakuninConfig, Value, file_finder::FileFinder};

use super::{ConfigOptions, DsotConfig, Result};

pub static CONFIG_FILE_NAME: &'static str = ".dsot";

pub static GLOBAL_FILE_LAYER_NAME: &'static str = "global";
pub static LOCAL_FILE_LAYER_NAME: &'static str = "local";
pub static CUSTOM_FILE_LAYER_NAME: &'static str = "custom";

impl DsotConfig {
    pub fn load(options: ConfigOptions, base_value: Value) -> Result<DsotConfig> {
        let mut handler = BakuninConfig::new().with_memory_layer("base", base_value.clone());

        if options.search {
            let global_search = FileFinder::new(CONFIG_FILE_NAME)
                .with_supported_extensions()
                .with_user_home()
                .with_user_config()
                .find_first(true)?;

            handler.add_file_layer(GLOBAL_FILE_LAYER_NAME, global_search.path)?;

            let local_search = FileFinder::new(CONFIG_FILE_NAME)
                .with_supported_extensions()
                .with_working_directory()
                .find_first(true)?;

            handler.add_file_layer(LOCAL_FILE_LAYER_NAME, local_search.path)?;
        }

        if options.create {
            if let Some(cfg) = handler.get_layer(GLOBAL_FILE_LAYER_NAME) {
                if !cfg.has_value() {
                    cfg.write_value(&base_value)?;
                }
            }
        }

        if let Some(path) = options.config_path {
            handler.add_file_layer(CUSTOM_FILE_LAYER_NAME, std::path::PathBuf::from(path))?;
        }

        if options.use_env {
            handler.add_environment_layer("env", "DSOT_");
        }

        let inner = handler.build_value(true)?;

        Ok(Self {
            data_location: load_data_folder(inner.get("data_folder")),
            user: inner.get("user").into_string_or("main".to_string()),
            handler,
            inner,
        })
    }
}

fn load_data_folder(v: Value) -> PathBuf {
    match v.try_into_string() {
        Ok(path) => PathBuf::from(path),
        _ => match dirs::home_dir() {
            Some(home) => home.join(CONFIG_FILE_NAME),
            None => PathBuf::from("./").join(CONFIG_FILE_NAME),
        },
    }
}
