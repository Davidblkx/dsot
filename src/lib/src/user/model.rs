use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

use dsot_db_sync::{DatabaseManager, DsotDatabase};

use super::local::{LocalUser, LocalUserCredentials};
use crate::error::{DsotError, Result};

#[derive(Debug, Clone)]
enum User {
    Empty,
    Local(LocalUser),
    // We need to eventually have a Remote(..)
}

#[derive(Debug, Clone)]
pub struct DsotUser {
    user: Arc<RwLock<User>>,
}

impl DsotUser {
    pub fn empty() -> Self {
        Self {
            user: Arc::new(RwLock::new(User::Empty)),
        }
    }

    pub fn local(path: PathBuf, credentials: LocalUserCredentials) -> Result<Self> {
        let user = LocalUser::load(path, credentials)?;

        Ok(Self {
            user: Arc::new(RwLock::new(User::Local(user))),
        })
    }

    pub fn logout(&self) -> Result<()> {
        let mut lock = self.user.write().unwrap();
        *lock = User::Empty;
        Ok(())
    }

    pub fn login_local(&self, path: PathBuf, credentials: LocalUserCredentials) -> Result<()> {
        let user = LocalUser::load(path, credentials)?;
        let mut lock = self.user.write().unwrap();
        *lock = User::Local(user);
        Ok(())
    }

    pub fn db_manager(&self) -> Result<DatabaseManager> {
        let user = self.user.read().unwrap();
        match &*user {
            User::Empty => Err(DsotError::InvalidUser),
            User::Local(user) => Ok(user.db_manager().clone()),
        }
    }

    pub async fn open_db(&self) -> Result<DsotDatabase> {
        let manager = self.db_manager()?;

        let db = manager.open_database().await?;

        Ok(db)
    }

    pub fn protect(&self, password: impl AsRef<str>) -> Result<()> {
        let user = self.user.read().unwrap();
        match &*user {
            User::Empty => Ok(()),
            User::Local(user) => user.set_password(password),
        }
    }
}
