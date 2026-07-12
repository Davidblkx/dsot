pub mod configs;
pub mod logger;
pub mod user_manager;

pub mod core;
pub mod error;
pub mod state;

pub use dsot_config;
pub use dsot_db_sync;
pub use dsot_model;
pub use uuid;

#[derive(Debug, thiserror::Error)]
pub enum DsotStateInitError {
    #[error("Error initializing logger: {0}")]
    LogInitError(#[from] fern::InitError),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Config error: {0}")]
    ConfigError(#[from] dsot_config::DsotConfigError),
    #[error("Error opening user db: {0}")]
    OpenDBError(#[from] dsot_db_sync::DBSyncError),
    #[error("Error creating network: {0}")]
    NetworkError(#[from] dsot_network::DsotNetworkError),
}

pub type DsotStateConfig = dsot_config::DsotConfig<configs::ConfigValue>;

#[derive(Clone)]
pub struct DsotState {
    pub config: DsotStateConfig,
    pub user_manager: user_manager::UserManager,
    pub db: dsot_db_sync::DatabaseManager,
    pub network: Option<dsot_network::DsotNetwork>,
}

pub struct DsotStateInitOptions {
    pub debug: bool,
    pub config_file: Option<String>,
    pub is_mobile: bool,
}

impl DsotStateInitOptions {
    pub fn new() -> Self {
        Self {
            debug: false,
            config_file: None,
            is_mobile: false,
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
    pub async fn init(options: DsotStateInitOptions) -> Result<Self, DsotStateInitError> {
        if options.debug {
            let file = if !options.is_mobile {
                let date_now = chrono::Local::now().format("%Y_%m_%d_%H_%M").to_string();
                match sysdirs::temp_dir() {
                    Some(p) => Some(p.join(format!("dsot_logs.{}.txt", date_now))),
                    None => None,
                }
            } else {
                None
            };
            logger::init_log(logger::LogLevel::Debug, file)?;
        }

        let config = if options.is_mobile {
            configs::load_mobile_config()?
        } else {
            configs::load_config(&options.config_file)?
        };
        let user_manager = user_manager::UserManager::open(&config.data_dir)?;

        if !options.debug {
            logger::init_log(
                (&config.value.log_level).into(),
                config.value.log_file.clone(),
            )?;
        }

        let db = user_manager.open_user_db(&config.value.user.as_str())?;

        let network = if config.value.use_network {
            Some(
                dsot_network::DsotNetwork::init(dsot_network::NetworkInitOptions {
                    data_folder: config.data_dir.clone(),
                    config: config.value.network_config.clone(),
                    manager: std::sync::Arc::new(user_manager.clone()),
                })
                .await?,
            )
        } else {
            None
        };

        Ok(DsotState {
            config,
            user_manager,
            db,
            network,
        })
    }
}
