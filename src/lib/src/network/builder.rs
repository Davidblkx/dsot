use iroh::Endpoint;
use std::sync::Arc;

use super::{DsotNetwork, state::NetworkState};
use crate::{
    core::{cap::Capability, config::DsotAppConfig},
    error::Result,
    repository::DsotRepository,
    state::DsotState,
};

#[derive(Debug, Clone)]
pub struct NetworkBuilder {
    pub config: Arc<DsotAppConfig>,
    pub repo: DsotRepository,
    pub cap: Capability,
    pub state: DsotState,
}

impl NetworkBuilder {
    pub fn lazy_connect(self) -> DsotNetwork {
        self.into()
    }

    pub async fn connect(self) -> Result<DsotNetwork> {
        let state = self.connect_router().await?;
        Ok(DsotNetwork::new(state))
    }

    pub(crate) async fn connect_router(self) -> Result<NetworkState> {
        if !self.cap.can_network_access() || !self.config.value.use_network {
            ::log::debug!("Network access disabled");
            return Ok(NetworkState::Closed);
        }

        let key = self.load_network_key()?;

        let endpoint = Endpoint::builder(iroh::endpoint::presets::N0)
            .secret_key(key)
            .bind()
            .await?;

        let router = super::router::build_router(self, endpoint).await;
        Ok(NetworkState::Open(router.spawn()))
    }
}
