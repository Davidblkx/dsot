pub mod devices;
pub mod inbox;

use crate::{
    core::{config::DsotAppConfig, init::DsotCoreInitOptions},
    error::Result,
    repository::DsotRepository,
    state::devices::RemoteDevices,
};

#[derive(Debug, Clone)]
pub struct DsotState {
    pub devices: RemoteDevices,
}

impl DsotCoreInitOptions {
    pub async fn init_state(
        &self,
        _config: &DsotAppConfig,
        repo: &DsotRepository,
    ) -> Result<DsotState> {
        Ok(DsotState {
            devices: RemoteDevices::from_repository(repo).await?,
        })
    }
}
