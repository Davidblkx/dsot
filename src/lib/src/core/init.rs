use std::sync::Arc;

use super::{cap::Capability, model::DsotCore};
use crate::{error::Result, jobs::JobManager, network::builder::NetworkBuilder};

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

    pub async fn initialize(self) -> Result<DsotCore> {
        let has_debug_logger = self.init_debug_logger()?;

        let config = Arc::new(self.load_config()?);

        if !has_debug_logger {
            self.init_logger_from_config(&config)?;
        }

        let repo = self.init_repository(&config).await?;
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
        })
    }
}
