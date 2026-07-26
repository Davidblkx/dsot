use std::path::PathBuf;

mod devices;
mod user;

impl_repository_shell!(LocalRepo {
    users: user::LocalUserRepo,
    devices: devices::LocalDeviceRepo,
});

impl LocalRepo {
    pub fn init(root: PathBuf) -> Self {
        Self {
            users: user::LocalUserRepo::new(root.clone()),
            devices: devices::LocalDeviceRepo::new(root),
        }
    }
}
