use crate::core::config::DsotAppConfig;

crate::bitflag!(NetworkCapability {
    0 => network "Can connect to the network"
});

impl From<&DsotAppConfig> for NetworkCapability {
    fn from(config: &DsotAppConfig) -> Self {
        let mut v = Self::new();

        if config.value.use_network {
            v.enable_network();
        }

        v
    }
}
