pub mod devices;
mod user;

impl_repository_shell!(NoopRepo {
    users: user::UserNoopRepository,
    devices: devices::DevicesNoopRepository,
});

impl NoopRepo {
    pub fn init() -> Self {
        Self {
            users: user::UserNoopRepository::new(),
            devices: devices::DevicesNoopRepository {},
        }
    }
}
