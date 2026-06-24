use crate::{NetworkCapability, NetworkInitOptions, Result};
use dsot_serde::serde_binary;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MachineInfo {
    pub cap: NetworkCapability,
    pub name: String,
    pub desc: String,
}

impl MachineInfo {
    pub fn new(options: &NetworkInitOptions) -> Self {
        let cap = NetworkCapability {
            db_sync: options.config.use_db_sync,
        };

        let cfg = &options.config;

        let name = cfg.public_name.clone().unwrap_or_default();
        let desc = cfg.public_desc.clone().unwrap_or_default();

        Self { cap, name, desc }
    }
}

serde_binary!(MachineInfo);
