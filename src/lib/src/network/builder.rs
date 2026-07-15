use iroh::protocol::DynProtocolHandler;
use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use super::DsotNetwork;
use crate::{
    core::{cap::Capability, config::DsotAppConfig},
    error::{DsotError, Result},
    repository::DsotRepository,
};

#[derive(Debug, Clone)]
pub struct NetworkBuilder {
    config: Arc<DsotAppConfig>,
    repo: DsotRepository,
    cap: Capability,
    protocols: Arc<RwLock<BTreeMap<Vec<u8>, Box<dyn DynProtocolHandler>>>>,
}

impl NetworkBuilder {
    pub fn new(config: Arc<DsotAppConfig>, repo: DsotRepository, cap: Capability) -> Self {
        Self {
            config,
            repo,
            cap,
            protocols: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    pub fn accept(&self, alpn: impl AsRef<[u8]>, handler: impl Into<Box<dyn DynProtocolHandler>>) {
        self.protocols
            .write()
            .unwrap()
            .insert(alpn.as_ref().to_vec(), handler.into());
    }

    pub fn connect(self) -> Result<DsotNetwork> {
        Err(DsotError::NetworkDisconnected)
    }
}
