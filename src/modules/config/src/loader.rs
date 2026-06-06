use std::path::PathBuf;

use bakunin_config::{BakuninConfig, Value, file_finder::FileFinder};

use super::{ConfigOptions, DsotConfig, Result};

pub static CONFIG_FILE_NAME: &'static str = ".dsot";

pub static GLOBAL_FILE_LAYER_NAME: &'static str = "global";
pub static LOCAL_FILE_LAYER_NAME: &'static str = "local";
pub static CUSTOM_FILE_LAYER_NAME: &'static str = "custom";

impl<'a, T: serde::Deserialize<'a> + serde::Serialize + Default> DsotConfig<T> {
    pub fn load(options: ConfigOptions, base_value: T) -> Result<DsotConfig<T>> {
        let value = Value::serialize(base_value)?;
        let mut handler = BakuninConfig::new().with_memory_layer("base", value.clone());

        if options.from_data_dir {
            if let Some(dir) = sysdirs::data_dir() {
                let path = dir.join(format!("{}.toml", CONFIG_FILE_NAME));
                handler.add_file_layer(GLOBAL_FILE_LAYER_NAME, path)?;
            } else {
                log::warn!("No data directory found, falling back to search");
            }
        } else if options.search {
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
                    cfg.write_value(&value)?;
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

        let data_dir = load_data_folder(inner.get("data_dir"));
        log::trace!("Using data_dir: {}", data_dir.display());

        Ok(Self {
            data_dir,
            value: match inner.clone().deserialize::<T>() {
                Ok(v) => v,
                Err(e) => {
                    ::log::warn!("Invalid config file: {}", e);
                    T::default()
                }
            },
            handler,
            inner,
        })
    }
}

fn load_data_folder(v: Value) -> PathBuf {
    match v.try_into_string() {
        Ok(path) => PathBuf::from(path),
        _ => match sysdirs::home_dir() {
            Some(home) => home.join(CONFIG_FILE_NAME),
            None => PathBuf::from("./").join(CONFIG_FILE_NAME),
        },
    }
}
