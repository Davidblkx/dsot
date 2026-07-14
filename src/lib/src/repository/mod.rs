mod local;
mod noop;
mod remote;
mod traits;

use std::sync::Arc;

use crate::{
    core::{config::DsotAppConfig, init::DsotCoreInitOptions},
    error::Result,
};

pub use traits::*;

#[derive(Debug, Clone)]
pub struct DsotRepository {
    pub users: Arc<dyn UserRepository>,
}

impl DsotCoreInitOptions {
    pub async fn init_repository(&self, config: &DsotAppConfig) -> Result<DsotRepository> {
        let user_id = config.value.user.clone();

        Ok(DsotRepository {
            users: Arc::new(noop::UserNoopRepository::new(user_id)),
        })
    }
}
