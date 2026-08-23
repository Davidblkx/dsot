#[macro_use]
mod macros;

mod local;
mod remote;
mod repo;

mod repos;

use crate::{
    core::{config::DsotAppConfig, init::DsotCoreInitOptions},
    error::Result,
    user::DsotUser,
};

pub use repos::{DeviceRepository, DsotRepository, InboxRepository, Repository, UserRepository};

impl DsotCoreInitOptions {
    pub async fn init_repository(
        &self,
        config: &DsotAppConfig,
        user: DsotUser,
    ) -> Result<DsotRepository> {
        DsotRepository::init(self, config, user).await
    }
}
