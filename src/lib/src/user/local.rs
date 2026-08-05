use std::path::{Path, PathBuf};

use dsot_db_sync::DatabaseManager;

use crate::{DsotCore, error::Result};

static CREDENTIALS_FILE: &'static str = "credentials.key";

#[derive(Debug, Clone)]
pub struct LocalUser {
    path: PathBuf,
    manager: DatabaseManager,
}

#[derive(Debug, PartialEq)]
pub enum LocalUserCredentials {
    HashV1(blake3::Hash),
    Empty,
    SkipValidation,
}

impl LocalUserCredentials {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        LocalUserCredentials::HashV1(blake3::hash(bytes))
    }

    pub fn from_string(str: impl AsRef<str>) -> Self {
        LocalUserCredentials::HashV1(blake3::hash(str.as_ref().as_bytes()))
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let bytes = std::fs::read(path)?;
        if bytes.len() > 0 {
            Ok(Self::from_bytes(bytes.as_slice()))
        } else {
            Ok(Self::Empty)
        }
    }

    pub fn write_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let bytes = match self {
            Self::HashV1(hash) => hash.as_slice().to_vec(),
            _ => vec![],
        };
        std::fs::write(path, bytes)?;
        Ok(())
    }
}

impl LocalUser {
    pub fn load(path: PathBuf, credentials: LocalUserCredentials) -> Result<Self> {
        let cred_file = path.join(CREDENTIALS_FILE);
        let manager = DatabaseManager::open_folder(path.as_path())?;

        let is_login_valid = if cred_file.exists() {
            match credentials {
                LocalUserCredentials::Empty => {
                    let bytes = std::fs::read(cred_file.as_path())?;
                    bytes.len() == 0
                }
                LocalUserCredentials::SkipValidation => true,
                credentials => {
                    let local_credentials = LocalUserCredentials::from_file(cred_file.as_path())?;
                    credentials == local_credentials
                }
            }
        } else {
            true
        };

        if !is_login_valid {
            return Err(crate::error::DsotError::InvalidUserPassword);
        }

        Ok(Self { path, manager })
    }

    pub fn set_password(&self, password: impl AsRef<str>) -> Result<()> {
        let cred_file = self.path.join(CREDENTIALS_FILE);

        // If password is empty string, we clear any current password
        if password.as_ref().len() == 0 {
            if cred_file.exists() {
                std::fs::remove_file(cred_file.as_path())?;
            }
            ::log::debug!("Password protection removed: {}", cred_file.display());
            return Ok(());
        }

        let credentials = LocalUserCredentials::from_string(password);
        credentials.write_file(cred_file)?;

        Ok(())
    }

    pub fn db_manager(&self) -> &'_ DatabaseManager {
        &self.manager
    }
}

pub trait LocalUserPathProvider {
    fn get_user_path(&self, user_id: impl AsRef<str>) -> PathBuf;
}

impl LocalUserPathProvider for DsotCore {
    fn get_user_path(&self, user_id: impl AsRef<str>) -> PathBuf {
        self.config.data_dir.join("users").join(user_id.as_ref())
    }
}
