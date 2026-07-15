use async_trait::async_trait;

use crate::{error::Result, repository::DeviceRepository, state::devices::RemoteDevice};

#[derive(Debug)]
pub struct DevicesNoopRepository;

#[async_trait]
impl DeviceRepository for DevicesNoopRepository {
    async fn list_devices(&self) -> Result<Vec<RemoteDevice>> {
        Ok(vec![])
    }
    async fn add_device(&self, _device: RemoteDevice) -> Result<()> {
        Ok(())
    }
    async fn remove_device(&self, _id: iroh::EndpointId) -> Result<()> {
        Ok(())
    }
}
