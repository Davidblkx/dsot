use std::path::PathBuf;

pub mod configs;
pub mod logger;
pub mod user_manager;

#[derive(Debug, thiserror::Error)]
pub enum DsotStateInitError {
    #[error("Error initializing logger: {0}")]
    LogInitError(#[from] fern::InitError),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Config error: {0}")]
    ConfigError(#[from] dsot_config::DsotConfigError),
}

#[derive(Clone)]
pub struct DsotState {
    pub config: dsot_config::DsotConfig<configs::ConfigValue>,
    pub user_manager: user_manager::UserManager,
}

pub struct DsotStateInitOptions {
    pub debug: bool,
    pub config_file: Option<String>,
}

impl DsotStateInitOptions {
    pub fn new() -> Self {
        Self {
            debug: false,
            config_file: None,
        }
    }

    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    pub fn config_file(mut self, config_file: Option<String>) -> Self {
        self.config_file = config_file;
        self
    }
}

impl DsotState {
    pub fn init(options: DsotStateInitOptions) -> Result<Self, DsotStateInitError> {
        if options.debug {
            logger::init_log(logger::LogLevel::Trace, Some(PathBuf::from("./logs.txt")))?;
        }

        let config = configs::load_config(&options.config_file)?;
        let user_manager = user_manager::UserManager::open(&config.value.user)?;

        if !options.debug {
            logger::init_log(
                (&config.value.log_level).into(),
                config.value.log_file.clone(),
            )?;
        }

        Ok(DsotState {
            config,
            user_manager,
        })
    }
}
