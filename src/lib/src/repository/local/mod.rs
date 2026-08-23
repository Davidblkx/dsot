use std::path::PathBuf;

use crate::user::DsotUser;

mod devices;
mod inbox;
mod user;

impl_repository!(LocalRepo {
    device: devices::LocalDeviceRepo,
    inbox: inbox::InboxLocalRepository,
    user: user::LocalUserRepo,
});

impl LocalRepo {
    pub fn init(root: PathBuf, user: DsotUser) -> Self {
        Self {
            user: user::LocalUserRepo::new(root.clone()),
            device: devices::LocalDeviceRepo::new(root),
            inbox: inbox::InboxLocalRepository::new(user),
        }
    }
}
