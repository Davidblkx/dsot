use crate::{
    core::DsotCore,
    network::{NetworkCapability, builder::NetworkBuilder},
};

/// Information from a network device
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct NetworkDeviceInfo {
    /// Public name of device defined in config
    ///
    /// when name is not defined, `sysinfo::System::name()` is used
    pub name: String,
    /// Public description of device defined in config
    ///
    /// when desc is not defined, `sysinfo::System::long_os_version()` is used
    pub desc: String,
    /// Network capabilities of the device
    pub cap: NetworkCapability,
}

impl NetworkDeviceInfo {
    pub(crate) fn new(builder: &NetworkBuilder) -> Self {
        let cap = NetworkCapability::from(&*builder.config);
        let (name, desc) = (
            builder.config.value.network_config.public_name.clone(),
            builder.config.value.network_config.public_desc.clone(),
        );

        let name = if let Some(name) = name {
            name
        } else if let Some(hostname) = sysinfo::System::name() {
            hostname
        } else {
            "unknown".to_string()
        };

        let desc = if let Some(desc) = desc {
            desc
        } else if let Some(os) = sysinfo::System::long_os_version() {
            os
        } else {
            "No description".to_string()
        };

        Self { name, desc, cap }
    }
}

pub trait NetworkDeviceInfoBuilder {
    /// Public information for current device
    fn get_current_device_info(&self) -> NetworkDeviceInfo;
}

impl NetworkDeviceInfoBuilder for DsotCore {
    fn get_current_device_info(&self) -> NetworkDeviceInfo {
        NetworkDeviceInfo::new(&self.into_builder())
    }
}
