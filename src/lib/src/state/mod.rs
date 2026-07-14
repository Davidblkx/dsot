pub mod devices;
pub mod inbox;
pub mod user;

use crate::{
    core::config::DsotAppConfig, core::init::DsotCoreInitOptions, error::Result,
    repository::DsotRepository,
};

#[derive(Debug, Clone)]
pub struct DsotState {
    pub user: user::UserState,
}

impl DsotCoreInitOptions {
    pub async fn init_state(
        &self,
        config: &DsotAppConfig,
        _repo: &DsotRepository,
    ) -> Result<DsotState> {
        Ok(DsotState {
            user: user::UserState::new(config.value.user.clone()),
        })
    }
}
