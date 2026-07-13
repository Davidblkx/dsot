use super::{cap::Capability, config::DsotAppConfig};

#[derive(Debug, Clone)]
pub struct DsotCore {
    pub cap: Capability,
    pub config: DsotAppConfig,
}
