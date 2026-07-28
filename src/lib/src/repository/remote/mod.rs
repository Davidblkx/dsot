use crate::{core::DsotCore, network::NetworkDevice};

mod devices;
mod user;

impl_repository_shell!(RemoteRepo {
    users: user::UserRemoteRepo,
    devices: devices::DevicesRemoteRepo,
});

impl RemoteRepo {
    pub fn init(core: &DsotCore, device: &NetworkDevice) -> Self {
        Self {
            users: user::UserRemoteRepo::new(core, device.clone()),
            devices: devices::DevicesRemoteRepo::new(core, device.clone()),
        }
    }
}
