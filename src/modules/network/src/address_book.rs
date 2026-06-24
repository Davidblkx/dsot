use std::{io::Read, path::PathBuf};

use super::error::*;

static FILE_NAME: &'static str = "address_book.toml";

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NetworkAddress {
    pub name: String,
    pub desc: String,
    pub address: iroh::EndpointId,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AddressBook {
    pub path: PathBuf,
}

impl AddressBook {
    pub fn read(&self) -> Result<Vec<NetworkAddress>> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }

        ::log::trace!("Reading address book from {}", self.path.display());
        let mut file = std::fs::File::open(&self.path)?;
        let mut file_bytes = Vec::new();
        file.read_to_end(&mut file_bytes)?;

        Ok(toml::from_slice(&file_bytes)?)
    }

    pub fn read_safe(&self) -> Vec<NetworkAddress> {
        match self.read() {
            Ok(addr) => addr,
            Err(e) => {
                ::log::debug!("Failed to read address book: {}", e);
                Vec::new()
            }
        }
    }

    pub fn write(&self, addresses: &[NetworkAddress]) -> Result<()> {
        ::log::trace!("Writing address book to {}", self.path.display());

        let serialized = toml::to_string(addresses)?;
        std::fs::write(&self.path, serialized)?;
        Ok(())
    }

    pub fn init(base: &PathBuf, file: &Option<String>) -> Self {
        let file_name = match file {
            Some(f) => match f.is_empty() {
                true => FILE_NAME,
                false => f,
            },
            None => FILE_NAME,
        };

        Self {
            path: base.join(file_name),
        }
    }
}
