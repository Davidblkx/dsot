use iroh::SecretKey;
use std::path::PathBuf;

use super::builder::NetworkBuilder;
use crate::error::Result;

const KEY_NAME: &'static str = "dsot_network.key";

struct NetworkKey;

impl NetworkKey {
    fn create_file_key(file: PathBuf) -> Result<SecretKey> {
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

    fn try_read_file_key(file: PathBuf) -> Result<SecretKey> {
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
                    return NetworkKey::create_file_key(file);
                }
            };

            Ok(SecretKey::from(array))
        } else {
            NetworkKey::create_file_key(file)
        }
    }
}

impl NetworkBuilder {
    pub(crate) fn load_network_key(&self) -> Result<SecretKey> {
        if !self.cap.can_disk_access() || !self.config.value.network_config.persist_key {
            return Ok(SecretKey::generate());
        }

        let file = match &self.config.value.network_config.key_file {
            Some(key) => PathBuf::from(key),
            None => self.config.data_dir.join(KEY_NAME),
        };

        NetworkKey::try_read_file_key(file)
    }
}
