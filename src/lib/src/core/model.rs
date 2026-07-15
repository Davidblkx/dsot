use std::sync::Arc;

use super::{cap::Capability, config::DsotAppConfig};
use crate::{repository::DsotRepository, state::DsotState};

#[derive(Debug, Clone)]
pub struct DsotCore {
    pub cap: Capability,
    pub config: Arc<DsotAppConfig>,
    pub repo: DsotRepository,
    pub state: DsotState,
}
