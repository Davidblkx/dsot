use async_trait::async_trait;

use crate::{DsotCore, error::Result, repository::DeviceRepository, state::devices::RemoteDevice};

#[async_trait]
pub trait RemoteDeviceOperations {
    async fn add_or_update_device(&self, device: RemoteDevice) -> Result<()>;
    async fn remove_device(&self, id: iroh::PublicKey) -> Result<()>;
}

#[async_trait]
impl RemoteDeviceOperations for DsotCore {
    async fn add_or_update_device(&self, device: RemoteDevice) -> Result<()> {
        self.repo.add_or_update_device(device).await?;
        let devices = self.repo.list_devices().await?;
        self.state.devices.writer.send_replace(devices);
        Ok(())
    }
    async fn remove_device(&self, id: iroh::PublicKey) -> Result<()> {
        self.repo.remove_device(id).await?;
        let devices = self.repo.list_devices().await?;
        self.state.devices.writer.send_replace(devices);
        Ok(())
    }
}
