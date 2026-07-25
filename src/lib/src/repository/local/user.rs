use async_trait::async_trait;
use std::path::PathBuf;

use crate::{
    error::{DsotError, Result},
    repository::UserRepository,
};

#[derive(Debug)]
pub struct LocalUser {
    root: PathBuf,
}

impl LocalUser {
    pub fn new<T: Into<PathBuf>>(root: T) -> LocalUser {
        Self {
            root: root.into().join("users"),
        }
    }
}

#[async_trait]
impl UserRepository for LocalUser {
    async fn load_user(&self, user: &str, pass: Option<String>) -> Result<String> {
        let user_path = self.root.join(user);
        if !user_path.exists() {
            std::fs::create_dir(&user_path)?;
            if let Some(pass) = pass.clone() {
                validate_password(user_path.clone(), pass, true)?;
            }
        }

        if let Some(pass) = pass {
            validate_password(user_path.clone(), pass, false)?;
        }

        match user_path.to_str() {
            Some(value) => Ok(value.to_string()),
            _ => Err(DsotError::InvalidUserPasswordFile(
                "Invalid user path".to_string(),
            )),
        }
    }

    async fn list_users(&self) -> Result<Vec<String>> {
        let mut list = Vec::new();

        for r in std::fs::read_dir(&self.root)? {
            let entry = r?;
            if entry.path().is_dir() {
                match entry.file_name().into_string() {
                    Ok(value) => list.push(value),
                    _ => {
                        log::warn!(
                            "Skipping non-string user directory: {}",
                            entry.file_name().display()
                        );
                    }
                };
            }
        }

        Ok(list)
    }
}

pub fn validate_password(user_path: PathBuf, pass: String, create: bool) -> Result<()> {
    let hash_pass = blake3::hash(pass.as_bytes());
    let key_file = user_path.join("user.key");
    if key_file.exists() {
        let file_bytes = std::fs::read(key_file)?;
        match blake3::Hash::from_slice(file_bytes.as_slice()) {
            Ok(file_hash) => {
                if file_hash != hash_pass {
                    return Err(DsotError::InvalidUserPassword);
                }
            }
            Err(e) => {
                return Err(DsotError::InvalidUserPasswordFile(e.to_string()));
            }
        }
    } else if create {
        std::fs::write(key_file, hash_pass.as_slice())?;

        return Ok(());
    }

    Err(DsotError::InvalidUserPasswordFile(
        "Password not defined".to_string(),
    ))
}
