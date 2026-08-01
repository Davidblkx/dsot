pub mod devices;
pub mod inbox;

use crate::{
    core::{config::DsotAppConfig, init::DsotCoreInitOptions},
    error::Result,
    repository::DsotRepository,
};

#[derive(Debug, Clone)]
pub struct DsotState {
    pub devices: devices::RemoteDevices,
    pub inbox: inbox::InboxState,
}

impl DsotCoreInitOptions {
    pub async fn init_state(
        &self,
        _config: &DsotAppConfig,
        repo: &DsotRepository,
    ) -> Result<DsotState> {
        Ok(DsotState {
            devices: devices::RemoteDevices::from_repository(repo).await?,
            inbox: inbox::InboxState::from_repository(repo).await?,
        })
    }
}
