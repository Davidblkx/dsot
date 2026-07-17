use iroh::Endpoint;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use tokio::sync::Mutex;

use super::{DsotRouter, builder::NetworkBuilder};
use crate::error::{DsotError, Result};

#[derive(Debug, Clone)]
pub struct DsotNetwork {
    router: DsotRouter,
    endpoint: Arc<Mutex<Option<Endpoint>>>,
    connected: Arc<AtomicBool>,
    builder: NetworkBuilder,
}

impl DsotNetwork {
    pub(crate) async fn new_connected(builder: NetworkBuilder) -> Result<Self> {
        let endpoint = builder.connect().await?;

        Ok(Self {
            builder,
            router: DsotRouter::default(),
            endpoint: Arc::new(Mutex::new(endpoint)),
            connected: Arc::new(AtomicBool::new(true)),
        })
    }

    pub(crate) fn new_lazy(builder: NetworkBuilder) -> Self {
        Self {
            builder,
            router: DsotRouter::default(),
            endpoint: Arc::new(Mutex::new(None)),
            connected: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn is_connected(&self) -> bool {
        self.connected.load(Ordering::Acquire)
    }

    pub async fn connect(&self) -> Result<Endpoint> {
        if self.is_connected() {
            let guard = self.endpoint.lock().await;
            if let Some(e) = &*guard {
                return Ok(e.clone());
            }
        }

        let mut guard = self.endpoint.lock().await;

        // Double check in case another task connected while we waited for the lock
        if let Some(e) = &*guard {
            return Ok(e.clone());
        }

        if let Some(endpoint) = self.builder.connect().await? {
            *guard = Some(endpoint.clone());
            self.connected.store(true, Ordering::Release);
            Ok(endpoint)
        } else {
            Err(DsotError::NetworkDisconnected)
        }
    }

    pub async fn disconnect(&self) {
        self.router.disconnect().await;

        let endpoint = {
            let mut guard = self.endpoint.lock().await;
            let e = std::mem::replace(&mut *guard, None);
            self.connected.store(false, Ordering::Release);
            e
        };

        if let Some(e) = endpoint {
            if e.is_closed() {
                return;
            }

            e.close().await;
        }
    }
}
