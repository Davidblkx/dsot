use super::repos::DefaultInboxRepo;
use crate::{core::DsotCore, network::NetworkDevice};

mod devices;
mod user;

impl_repository!(RemoteRepo {
    device: devices::DevicesRemoteRepo,
    inbox: DefaultInboxRepo,
    user: user::UserRemoteRepo,
});

impl RemoteRepo {
    pub fn init(core: &DsotCore, device: &NetworkDevice) -> Self {
        Self {
            user: user::UserRemoteRepo::new(core, device.clone()),
            device: devices::DevicesRemoteRepo::new(core, device.clone()),
            inbox: DefaultInboxRepo {},
        }
    }
}
