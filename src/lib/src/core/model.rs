use std::sync::Arc;

use super::{cap::Capability, config::DsotAppConfig};
use crate::{jobs::JobManager, network::DsotNetwork, repository::DsotRepository, state::DsotState};

/// DSOT core model, holding all the state and configuration for the DSOT system.
#[derive(Debug, Clone)]
pub struct DsotCore {
    /// Capabilities of the DSOT system.
    pub cap: Capability,
    /// Configuration for the DSOT system.
    pub config: Arc<DsotAppConfig>,
    /// Repository for the DSOT system.
    pub repo: DsotRepository,
    /// State for the DSOT system.
    pub state: DsotState,
    /// Network for the DSOT system.
    pub net: DsotNetwork,
    /// Job manager for the DSOT system.
    pub jobs: JobManager,
}
