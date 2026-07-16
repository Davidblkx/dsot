mod builder;
mod config;
mod key;
mod router;
pub mod sink;
mod state;

use iroh::{Endpoint, protocol::Router};
use std::sync::{Arc, RwLock};

pub use builder::NetworkBuilder;
pub use config::NetworkConfig;
use state::NetworkState;

use crate::error::{DsotError, Result};

#[derive(Debug, Clone, Default)]
pub struct DsotNetwork {
    state: Arc<RwLock<NetworkState>>,
}

impl DsotNetwork {
    fn new(state: NetworkState) -> Self {
        Self {
            state: Arc::new(RwLock::new(state)),
        }
    }

    pub async fn connect(&self) -> Result<Endpoint> {
        let value = {
            if let NetworkState::Open(router) = &*self.state.read().unwrap() {
                Some(router.endpoint().to_owned())
            } else {
                None
            }
        };

        match value {
            Some(endpoint) => Ok(endpoint),
            None => {
                self.consume_builder().await?;
                if let NetworkState::Open(router) = &*self.state.read().unwrap() {
                    Ok(router.endpoint().to_owned())
                } else {
                    Err(DsotError::NetworkDisconnected)
                }
            }
        }
    }

    async fn consume_builder(&self) -> Result<()> {
        let old_value = {
            let mut guard = self.state.write().unwrap();

            std::mem::replace(&mut *guard, NetworkState::Closed)
        };

        match old_value {
            NetworkState::Closed => Err(DsotError::NetworkDisconnected),
            NetworkState::Open(_) => Err(DsotError::NetworkDoubleConnection),
            NetworkState::Ready(builder) => {
                let state = builder.connect_router().await?;

                let mut writer = self.state.write().unwrap();
                *writer = state;

                Ok(())
            }
        }
    }
}

impl From<NetworkBuilder> for DsotNetwork {
    fn from(value: NetworkBuilder) -> Self {
        Self::new(NetworkState::Ready(value))
    }
}

impl From<Router> for DsotNetwork {
    fn from(value: Router) -> Self {
        Self::new(NetworkState::Open(value))
    }
}
