use crate::core::config::DsotAppConfig;

crate::bitflag!(NetworkCapability {
    0 => network "network access",
    1 => sync "sync database with other devices"
});

impl From<&DsotAppConfig> for NetworkCapability {
    fn from(config: &DsotAppConfig) -> Self {
        let mut v = Self::new();

        if config.value.use_network {
            v.enable_network();
        }

        if config.value.use_db_sync {
            v.enable_sync();
        }

        v
    }
}
