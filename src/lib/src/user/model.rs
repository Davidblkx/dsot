use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};
use tokio::sync::watch::{self, Receiver, Sender};

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
    is_loggin: Receiver<bool>,
    update_login: Arc<Sender<bool>>,
}

impl DsotUser {
    fn new(user: User, is_loggin: bool) -> Self {
        let (update_login, is_loggin) = watch::channel(is_loggin);
        Self {
            user: Arc::new(RwLock::new(user)),
            is_loggin,
            update_login: Arc::new(update_login),
        }
    }

    pub fn empty() -> Self {
        Self::new(User::Empty, false)
    }

    fn update_login_status(&self, status: bool) {
        self.update_login.send_if_modified(|l| {
            if &status != l {
                *l = status;
                true
            } else {
                false
            }
        });
    }

    pub fn local(path: PathBuf, credentials: LocalUserCredentials) -> Result<Self> {
        let user = LocalUser::load(path, credentials)?;
        Ok(Self::new(User::Local(user), false))
    }

    pub fn logout(&self) -> Result<()> {
        let mut lock = self.user.write().unwrap();
        *lock = User::Empty;
        self.update_login_status(false);
        Ok(())
    }

    pub fn login_local(&self, path: PathBuf, credentials: LocalUserCredentials) -> Result<()> {
        let user = LocalUser::load(path, credentials)?;
        let mut lock = self.user.write().unwrap();
        *lock = User::Local(user);
        self.update_login_status(true);
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

    pub fn is_logged_in(&self) -> &'_ Receiver<bool> {
        &self.is_loggin
    }
}
