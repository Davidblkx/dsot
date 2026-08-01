mod devices;
pub mod inbox;
mod user;

impl_repository_shell!(NoopRepo {
    users: user::UserNoopRepository,
    devices: devices::DevicesNoopRepository,
    inbox: inbox::NoopInboxRepository,
});

impl NoopRepo {
    pub fn init() -> Self {
        Self {
            users: user::UserNoopRepository::new(),
            devices: devices::DevicesNoopRepository {},
            inbox: inbox::NoopInboxRepository {},
        }
    }
}
