use std::{io::Read, path::PathBuf};

use super::error::*;

static FILE_NAME: &'static str = "address_book.toml";

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NetworkAddress {
    pub name: String,
    pub desc: String,
    pub address: iroh::EndpointId,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NetworkAddressBook {
    pub addresses: Vec<NetworkAddress>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AddressBook {
    pub path: PathBuf,
}

impl AddressBook {
    pub fn read(&self) -> Result<NetworkAddressBook> {
        if !self.path.exists() {
            return Ok(NetworkAddressBook { addresses: vec![] });
        }

        ::log::trace!("Reading address book from {}", self.path.display());
        let mut file = std::fs::File::open(&self.path)?;
        let mut file_bytes = Vec::new();
        file.read_to_end(&mut file_bytes)?;

        Ok(toml::from_slice(&file_bytes)?)
    }

    pub fn read_safe(&self) -> NetworkAddressBook {
        match self.read() {
            Ok(addr) => addr,
            Err(e) => {
                ::log::debug!("Failed to read address book: {}", e);
                NetworkAddressBook { addresses: vec![] }
            }
        }
    }

    pub fn write_addresses(&self, addresses: Vec<NetworkAddress>) -> Result<()> {
        let mut book = self.read()?;
        book.addresses = addresses;

        ::log::trace!("Writing address book to {}", self.path.display());

        let serialized = toml::to_string(&book)?;
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
