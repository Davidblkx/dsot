use iroh::{Endpoint, SecretKey, protocol::Router};
use std::path::PathBuf;

use crate::NetworkCapability;

use super::{DsotNetwork, NetworkInitOptions, Result};
use crate::protocols::{RegisterInfoProtocol, RegisterSyncProtocolV1};

const KEY_NAME: &'static str = "dsot_network.key";

impl DsotNetwork {
    pub async fn init(options: NetworkInitOptions) -> Result<DsotNetwork> {
        let key = Self::load_key(&options).await?;

        let capabilities = NetworkCapability {
            db_sync: options.config.use_db_sync,
        };

        let endpoint = Endpoint::builder(iroh::endpoint::presets::N0)
            .secret_key(key)
            .bind()
            .await?;

        let router_builder = Router::builder(endpoint.clone())
            .register_info_protocol(&options)
            .register_sync_protocol_v1(&options);

        let router = router_builder.spawn();

        let address_book =
            super::AddressBook::init(&options.data_folder, &options.config.address_book);

        Ok(DsotNetwork {
            endpoint,
            router,
            address_book,
            capabilities,
        })
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
