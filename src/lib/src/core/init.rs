use std::sync::Arc;

use super::{cap::Capability, model::DsotCore};
use crate::{
    error::Result,
    jobs::JobManager,
    network::builder::NetworkBuilder,
    user::{DsotUser, LocalUserCredentials},
};

/// Options to initialize DSOT
#[derive(Debug, Clone)]
pub struct DsotCoreInitOptions {
    /// Enable debug mode
    ///
    /// Debug mode always set log level to debug and writes logs to text file
    ///
    /// Logger is also initialed before configuration, allowing it to catch problems earlier
    pub debug: bool,
    /// Path to configuration files, this file overwrites all other configs except for environment
    pub config_file: Option<String>,
    /// System capability
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

    /// Initialize DSOT core
    ///
    /// Initialization order:
    /// - init debug logger (if debug mode is enabled)
    /// - load configuration
    /// - init logger (if debug mode is disabled)
    /// - initialize repository, state, and network
    pub async fn initialize(self) -> Result<DsotCore> {
        let has_debug_logger = self.init_debug_logger()?;

        let config = Arc::new(self.load_config()?);

        if !has_debug_logger {
            self.init_logger_from_config(&config)?;
        }

        let user = DsotUser::empty();
        if let Some(id) = &config.value.user {
            let user_folder = config.data_dir.join(id);
            if user_folder.exists() {
                user.login_local(user_folder, LocalUserCredentials::SkipValidation)?;
            } else {
                ::log::warn!("User folder not found: {}", user_folder.display());
            }
        }

        let repo = self.init_repository(&config, user.clone()).await?;
        let state = self.init_state(&config, &repo).await?;

        let net = {
            let builder = NetworkBuilder {
                cap: self.cap,
                config: config.clone(),
                repo: repo.clone(),
                state: state.clone(),
            };

            if config.value.network_config.lazy {
                builder.into_lazy_connection()
            } else {
                builder.into_connection().await?
            }
        };

        Ok(DsotCore {
            cap: self.cap,
            config,
            repo,
            state,
            net,
            jobs: JobManager::new(),
            user,
        })
    }
}
