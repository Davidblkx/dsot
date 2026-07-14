use async_trait::async_trait;

use crate::{error::Result, state::devices::RemoteDevice};

#[async_trait]
pub trait DeviceRepository: Send + Sync + std::fmt::Debug {
    async fn list_devices(&self) -> Result<Vec<RemoteDevice>>;
    async fn add_device(&self, device: RemoteDevice) -> Result<()>;
    async fn remove_device(&self, id: iroh::EndpointId) -> Result<()>;
}
