use std::sync::Arc;
use tokio::sync::watch;

use crate::{core::DsotCore, error::Result, repository::*};

pub type User = String;
static DEFAULT_USER: &'static str = "root";

#[derive(Debug, Clone)]
pub struct UserState {
    pub user: watch::Receiver<User>,
    writer: Arc<watch::Sender<User>>,
}

impl UserState {
    pub async fn new(repo: &DsotRepository) -> Result<Self> {
        let id = match repo.load_user(DEFAULT_USER, None).await {
            Ok(id) => id,
            Err(e) => {
                ::log::warn!("Failed to load default user: {}", e);
                DEFAULT_USER.to_string()
            }
        };

        let (writer, user) = watch::channel(id);
        Ok(Self {
            user,
            writer: Arc::new(writer),
        })
    }

    pub fn id(&self) -> String {
        self.user.borrow().clone()
    }
}

impl DsotCore {
    pub async fn load_user(&self, user: &str, pass: Option<String>) -> Result<()> {
        let id = self.repo.load_user(user, pass).await?;
        self.state.user.writer.send_if_modified(|v| {
            if v != &id {
                *v = id;
                true
            } else {
                false
            }
        });

        Ok(())
    }
}
