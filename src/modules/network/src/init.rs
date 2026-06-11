use dsot_db_sync::sync::iroh_protocol::{DBSyncProtocol, DSOT_DB_SYNC_ALPN_V1};
use iroh::{Endpoint, SecretKey, protocol::Router};
use std::path::PathBuf;

use super::{DsotNetwork, NetworkInitOptions, Result};

const KEY_NAME: &'static str = "dsot_network.key";

impl DsotNetwork {
    pub async fn init(options: &NetworkInitOptions) -> Result<DsotNetwork> {
        let key = Self::load_key(&options).await?;

        let endpoint = Endpoint::builder(iroh::endpoint::presets::N0)
            .secret_key(key)
            .bind()
            .await?;

        let mut router_builder = Router::builder(endpoint.clone());

        if options.config.use_db_sync {
            router_builder =
                router_builder.accept(DSOT_DB_SYNC_ALPN_V1, DBSyncProtocol::new(options.manager));
        }

        let router = router_builder.spawn();

        Ok(DsotNetwork { endpoint, router })
    }

    async fn load_key(o: &NetworkInitOptions) -> Result<SecretKey> {
        if !o.config.persist_key {
            return Ok(SecretKey::generate());
        }

        let file = match &o.config.key_file {
            Some(key) => PathBuf::from(key),
            None => o.data_folder.join(KEY_NAME),
        };

        if file.exists() {
            let bytes = std::fs::read(&file)?;

            // Try converting raw bytes into an Iroh SecretKey structure
            let array: [u8; 32] = match bytes.try_into() {
                Ok(array) => array,
                Err(_) => {
                    ::log::warn!(
                        "Invalid key file {0} format: Expected exactly 32 bytes",
                        file.display()
                    );
                    return Self::create_file_key(file).await;
                }
            };

            Ok(SecretKey::from(array))
        } else {
            Self::create_file_key(file).await
        }
    }

    async fn create_file_key(file: PathBuf) -> Result<SecretKey> {
        let new_key = SecretKey::generate();

        let raw_bytes = new_key.to_bytes();

        match std::fs::write(&file, raw_bytes) {
            Ok(_) => Ok(new_key),
            Err(e) => {
                ::log::warn!("Failed to write key file {0}: {1}", file.display(), e);
                Ok(new_key)
            }
        }
    }
}
