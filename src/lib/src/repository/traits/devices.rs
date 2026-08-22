use crate::{error::Result, state::devices::RemoteDevice};

declare_repository_unit!(Device {
    async fn list_devices(&self) -> Result<Vec<RemoteDevice>>; Ok(vec![]);
    async fn add_device(&self, device: RemoteDevice) -> Result<bool>; Ok(false);
    async fn remove_device(&self, id: iroh::EndpointId) -> Result<()>; Ok(());
});

pub struct MyRepository {
    device: DefaultDeviceRepo,
}

pub trait MyRepositoryTrait {
    type DeviceImpl: DeviceRepository;

    fn device(&self) -> &Self::DeviceImpl;
}

impl MyRepositoryTrait for MyRepository {
    type DeviceImpl = DefaultDeviceRepo;

    fn device(&self) -> &Self::DeviceImpl {
        &self.device
    }
}
