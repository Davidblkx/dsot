use iroh::Endpoint;
use std::sync::Arc;

use crate::{
    core::{DsotCore, cap::Capability, config::DsotAppConfig},
    error::Result,
    network::DsotNetwork,
    repository::DsotRepository,
    state::DsotState,
};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(crate) struct NetworkBuilder {
    pub config: Arc<DsotAppConfig>,
    pub repo: DsotRepository,
    pub cap: Capability,
    pub state: DsotState,
}

impl NetworkBuilder {
    pub async fn connect(&self) -> Result<Option<Endpoint>> {
        if !self.cap.can_network_access() || !self.config.value.use_network {
            ::log::debug!("Network access disabled");
            return Ok(None);
        }

        let key = self.load_network_key()?;

        let endpoint = Endpoint::builder(iroh::endpoint::presets::N0)
            .secret_key(key)
            .bind()
            .await?;

        Ok(Some(endpoint))
    }

    pub fn into_lazy_connection(self) -> DsotNetwork {
        DsotNetwork::new_lazy(self)
    }

    pub async fn into_connection(self) -> Result<DsotNetwork> {
        DsotNetwork::new_connected(self).await
    }
}

impl DsotCore {
    pub(crate) fn into_builder(&self) -> NetworkBuilder {
        NetworkBuilder {
            cap: self.cap,
            config: self.config.clone(),
            repo: self.repo.clone(),
            state: self.state.clone(),
        }
    }
}
