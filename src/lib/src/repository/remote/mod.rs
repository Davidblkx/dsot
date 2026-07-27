use crate::{core::DsotCore, network::NetworkDevice};

mod user;

impl_repository_shell!(RemoteRepo {
    users: user::UserRemoteRepo,
    devices: super::noop::devices::DevicesNoopRepository,
});

impl RemoteRepo {
    pub fn init(core: &DsotCore, device: &NetworkDevice) -> Self {
        Self {
            users: user::UserRemoteRepo::new(core, device.clone()),
            devices: super::noop::devices::DevicesNoopRepository {},
        }
    }
}
