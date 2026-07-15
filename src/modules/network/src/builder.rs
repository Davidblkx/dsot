use iroh::{
    Endpoint,
    protocol::{DynProtocolHandler, IncomingFilter, Router, RouterBuilder},
};
use std::sync::Mutex;

use crate::error::{DsotNetworkError, Result};

pub struct DsotNetworkBuilder {
    router: Mutex<Option<RouterBuilder>>,
}

impl DsotNetworkBuilder {
    pub fn new(endpoint: Endpoint) -> Self {
        Self {
            router: Mutex::new(Some(Router::builder(endpoint))),
        }
    }

    pub fn accept(
        &self,
        alpn: impl AsRef<[u8]>,
        handler: impl Into<Box<dyn DynProtocolHandler>>,
    ) -> Result<()> {
        let mut guard = self.router.lock().unwrap();

        if let Some(builder) = guard.take() {
            let builder = builder.accept(alpn, handler);
            *guard = Some(builder);
            Ok(())
        } else {
            Err(DsotNetworkError::Unknown)
        }
    }

    pub fn set_filter(&self, filter: IncomingFilter) -> Result<()> {
        let mut guard = self.router.lock().unwrap();

        if let Some(builder) = guard.take() {
            let builder = builder.incoming_filter(filter);
            *guard = Some(builder);
            Ok(())
        } else {
            Err(DsotNetworkError::Unknown)
        }
    }
}
