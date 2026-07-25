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
        _config: &DsotAppConfig,
        repo: &DsotRepository,
    ) -> Result<DsotState> {
        let user = user::UserState::new(repo).await?;

        Ok(DsotState { user })
    }
}
