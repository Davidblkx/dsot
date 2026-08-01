use std::path::PathBuf;

use crate::user::DsotUser;

mod devices;
mod inbox;
mod user;

impl_repository_shell!(LocalRepo {
    users: user::LocalUserRepo,
    devices: devices::LocalDeviceRepo,
    inbox: inbox::InboxLocalRepository,
});

impl LocalRepo {
    pub fn init(root: PathBuf, user: DsotUser) -> Self {
        Self {
            users: user::LocalUserRepo::new(root.clone()),
            devices: devices::LocalDeviceRepo::new(root),
            inbox: inbox::InboxLocalRepository::new(user),
        }
    }
}
