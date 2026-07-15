mod builder;
mod config;
mod state;

pub use config::NetworkConfig;

#[derive(Debug, Clone, Default)]
pub struct DsotNetwork {
    connection: state::NetworkState,
}

impl DsotNetwork {
    pub fn new() -> Self {
        Self {
            connection: state::NetworkState::default(),
        }
    }

    pub fn can_use(&self) -> bool {
        if let state::NetworkState::Open(router) = &self.connection {
            !router.is_shutdown()
        } else {
            false
        }
    }
}
