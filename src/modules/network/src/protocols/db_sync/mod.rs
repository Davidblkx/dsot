pub mod iroh_sync_node;
pub mod status_protocol;
pub mod sync_protocol;

pub use status_protocol::{DBSyncStatusProtocol, DSOT_DB_SYNC_STATUS_ALPN};
pub use sync_protocol::{DBSyncProtocol, DSOT_DB_SYNC_ALPN_V1};

pub trait RegisterDBSyncProtocols {
    fn register_db_sync_protocol(self, options: &crate::NetworkInitOptions) -> Self;
}

impl RegisterDBSyncProtocols for iroh::protocol::RouterBuilder {
    fn register_db_sync_protocol(self, options: &crate::NetworkInitOptions) -> Self {
        if options.config.use_db_sync {
            self.accept(
                DSOT_DB_SYNC_STATUS_ALPN,
                DBSyncStatusProtocol::new(options.manager.clone()),
            )
            .accept(
                DSOT_DB_SYNC_ALPN_V1,
                DBSyncProtocol::new(options.manager.clone()),
            )
        } else {
            self
        }
    }
}
