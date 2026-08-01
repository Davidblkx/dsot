use std::sync::Arc;

use tokio::sync::watch;

use crate::{
    error::Result,
    repository::{DeviceRepository, DsotRepository},
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoteDevice {
    pub id: iroh::EndpointId,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct RemoteDevices {
    pub devices: watch::Receiver<Vec<RemoteDevice>>,
    pub(crate) writer: Arc<watch::Sender<Vec<RemoteDevice>>>,
}

impl RemoteDevices {
    fn new(initial: Vec<RemoteDevice>) -> Self {
        let (writer, devices) = watch::channel(initial);
        Self {
            devices,
            writer: Arc::new(writer),
        }
    }

    pub async fn from_repository(repo: &DsotRepository) -> Result<Self> {
        let initial = repo.list_devices().await?;
        Ok(Self::new(initial))
    }
}
