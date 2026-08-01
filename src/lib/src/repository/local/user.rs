use async_trait::async_trait;
use std::path::PathBuf;

use crate::{error::Result, repository::UserRepository};

#[derive(Debug)]
pub struct LocalUserRepo {
    root: PathBuf,
}

impl LocalUserRepo {
    pub fn new<T: Into<PathBuf>>(root: T) -> LocalUserRepo {
        Self {
            root: root.into().join("users"),
        }
    }
}

#[async_trait]
impl UserRepository for LocalUserRepo {
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
