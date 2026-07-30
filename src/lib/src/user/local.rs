use std::path::{Path, PathBuf};

use dsot_db_sync::DatabaseManager;

use crate::error::Result;

#[derive(Debug, Clone)]
pub struct LocalUser {
    path: PathBuf,
    secure: bool,
    db_manager: DatabaseManager,
}

pub enum LocalUserCredentials {
    HashV1(blake3::Hash),
    Empty,
    SkipValidation,
}

impl LocalUserCredentials {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        LocalUserCredentials::HashV1(blake3::hash(bytes))
    }

    pub fn from_string(str: impl ToString) -> Self {
        LocalUserCredentials::HashV1(blake3::hash(str.to_string().as_bytes()))
    }

    pub fn from_file(path: &Path) -> Result<Self> {
        let bytes = std::fs::read(path)?;
        if bytes.len() > 0 {
            Ok(Self::from_bytes(bytes.as_slice()))
        } else {
            Ok(Self::Empty)
        }
    }

    pub fn write_file(&self, path: &Path) -> Result<()> {
        let bytes = match self {
            Self::HashV1(hash) => hash.as_slice().iter().map(|n| *n).collect::<Vec<u8>>(),
            _ => vec![],
        };
        std::fs::write(path, bytes)?;
        Ok(())
    }
}
