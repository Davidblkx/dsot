use async_trait::async_trait;

use crate::{
    core::DsotCore,
    error::Result,
    network::{
        NetworkDevice,
        protocols::server::{NetworkValidator, TokenValidator},
    },
    repository::UserRepository,
};

#[derive(Debug)]
pub struct UserRemoteRepo {
    validator: TokenValidator,
    device: NetworkDevice,
}

impl UserRemoteRepo {
    pub fn new(core: &DsotCore, device: NetworkDevice) -> Self {
        Self {
            validator: core.get_validator(),
            device,
        }
    }
}

#[async_trait]
impl UserRepository for UserRemoteRepo {
    async fn list_users(&self) -> Result<Vec<String>> {
        self.device.users(self.validator).list().await
    }
}
