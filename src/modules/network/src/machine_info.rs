use crate::{NetworkCapability, NetworkInitOptions, Result};
use dsot_serde::serde_binary;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
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

        let name = get_machine_name(&cfg.public_name);
        let desc = get_machine_desc(&cfg.public_desc);

        Self { cap, name, desc }
    }
}

serde_binary!(MachineInfo);

fn get_machine_name(name: &Option<String>) -> String {
    if let Some(name) = name {
        name.to_owned()
    } else if let Ok(hostname) = sys_info::hostname() {
        hostname
    } else if let Ok(os) = sys_info::os_type() {
        os
    } else {
        "unknown".to_string()
    }
}

fn get_machine_desc(name: &Option<String>) -> String {
    let mut desc = if let Some(d) = name {
        d.to_owned()
    } else {
        "%os_type% [%os_release%]".to_string()
    };

    if let Ok(os_type) = sys_info::os_type() {
        desc = desc.replace("%os_type%", &os_type);
    }

    if let Ok(os_release) = sys_info::os_release() {
        desc = desc.replace("%os_release%", &os_release);
    }

    desc
}
