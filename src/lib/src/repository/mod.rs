#[macro_use]
mod macros;

mod local;
mod noop;
mod remote;
mod repo;
mod traits;

use crate::{
    core::{config::DsotAppConfig, init::DsotCoreInitOptions},
    error::Result,
};

pub use repo::DsotRepository;
pub use traits::*;

impl DsotCoreInitOptions {
    pub async fn init_repository(&self, config: &DsotAppConfig) -> Result<DsotRepository> {
        DsotRepository::init(self, config).await
    }
}
