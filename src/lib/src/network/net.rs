use iroh::Endpoint;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use tokio::sync::{
    Mutex,
    watch::{self, Receiver, Sender},
};

use super::{DsotRouter, builder::NetworkBuilder};
use crate::error::{DsotError, Result};

#[derive(Debug, Clone)]
pub struct DsotNetwork {
    router: DsotRouter,
    endpoint: Arc<Mutex<Option<Endpoint>>>,
    connected: Arc<AtomicBool>,
    builder: NetworkBuilder,
    status_sender: Arc<Sender<bool>>,
    pub status: Receiver<bool>,
}

impl DsotNetwork {
    pub(crate) async fn new_connected(builder: NetworkBuilder) -> Result<Self> {
        let endpoint = builder.connect().await?;
        let connected = Arc::new(AtomicBool::new(endpoint.is_some()));
        let (sender, status) = watch::channel(endpoint.is_some());

        Ok(Self {
            router: DsotRouter::new(builder.clone()),
            builder,
            endpoint: Arc::new(Mutex::new(endpoint)),
            connected,
            status,
            status_sender: Arc::new(sender),
        })
    }

    pub(crate) fn new_lazy(builder: NetworkBuilder) -> Self {
        let (sender, status) = watch::channel(false);

        Self {
            router: DsotRouter::new(builder.clone()),
            builder,
            endpoint: Arc::new(Mutex::new(None)),
            connected: Arc::new(AtomicBool::new(false)),
            status,
            status_sender: Arc::new(sender),
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
            self.set_status(true);
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
            self.set_status(false);
            e
        };

        if let Some(e) = endpoint {
            if e.is_closed() {
                return;
            }

            e.close().await;
        }
    }

    #[inline]
    fn set_status(&self, value: bool) {
        self.connected.store(value, Ordering::Release);
        self.status_sender.send_if_modified(|v| {
            if v != &value {
                *v = value;
                true
            } else {
                false
            }
        });
    }
}
