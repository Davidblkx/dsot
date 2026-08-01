pub mod devices;
pub mod inbox;

use crate::{
    core::config::DsotAppConfig, core::init::DsotCoreInitOptions, error::Result,
    repository::DsotRepository,
};

#[derive(Debug, Clone)]
pub struct DsotState {}

impl DsotCoreInitOptions {
    pub async fn init_state(
        &self,
        _config: &DsotAppConfig,
        _repo: &DsotRepository,
    ) -> Result<DsotState> {
        Ok(DsotState {})
    }
}
