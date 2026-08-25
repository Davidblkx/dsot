use async_trait::async_trait;

use crate::{
    core::DsotCore,
    error::Result,
    network::{
        NetworkDevice,
        protocols::server::{NetworkValidator, TokenValidator},
    },
    repository::DeviceRepository,
    state::devices::RemoteDevice,
};

#[derive(Debug)]
pub struct DevicesRemoteRepo {
    validator: TokenValidator,
    device: NetworkDevice,
}

impl DevicesRemoteRepo {
    pub fn new(core: &DsotCore, device: NetworkDevice) -> Self {
        Self {
            validator: core.get_validator(),
            device,
        }
    }
}

#[async_trait]
impl DeviceRepository for DevicesRemoteRepo {
    async fn list_devices(&self) -> Result<Vec<RemoteDevice>> {
        self.device.devices(self.validator).list().await
    }
    async fn add_or_update_device(&self, device: RemoteDevice) -> Result<()> {
        self.device
            .devices(self.validator)
            .add_or_update(device)
            .await?;
        Ok(())
    }
    async fn remove_device(&self, id: iroh::EndpointId) -> Result<()> {
        self.device
            .devices(self.validator)
            .remove(id)
            .await
            .map(|_| ())
    }
}
