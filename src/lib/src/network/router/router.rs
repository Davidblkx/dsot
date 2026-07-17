use iroh::{
    Endpoint,
    protocol::{Router, RouterBuilder},
};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};

use crate::error::Result;

#[derive(Debug, Clone)]
pub struct DsotRouter {
    inner: Arc<Mutex<Option<Router>>>,
    connected: Arc<AtomicBool>,
}

impl Default for DsotRouter {
    fn default() -> Self {
        Self {
            inner: Arc::new(Mutex::new(None)),
            connected: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl DsotRouter {
    pub fn is_connected(&self) -> bool {
        self.connected.load(Ordering::Acquire)
    }

    pub async fn connect(&self, endpoint: Endpoint) -> Result<()> {
        if self.is_connected() {
            return Ok(());
        }

        let mut guard = self.inner.lock().unwrap();

        if guard.is_some() {
            return Ok(()); // Another thread beat us to it, just return!
        }

        let builder = RouterBuilder::new(endpoint);
        let router = builder.spawn();

        *guard = Some(router);

        self.connected.store(true, Ordering::Release);

        Ok(())
    }

    pub async fn disconnect(&self) {
        if !self.is_connected() {
            return;
        }

        let router = {
            let mut guard = self.inner.lock().unwrap();
            let r = std::mem::replace(&mut *guard, None);

            self.connected.store(false, Ordering::Release);
            r
        };

        if let Some(router) = router {
            match router.shutdown().await {
                Ok(_) => {}
                Err(e) => {
                    ::log::error!("Failed to shutdown router: {}", e);
                }
            };
        }
    }
}
