use std::sync::Arc;
use tokio::sync::watch;

use crate::{core::DsotCore, error::Result};

pub type User = String;

#[derive(Debug, Clone)]
pub struct UserState {
    pub user: watch::Receiver<User>,
    writer: Arc<watch::Sender<User>>,
}

impl UserState {
    pub fn new(initial: User) -> Self {
        let (writer, user) = watch::channel(initial);
        Self {
            user,
            writer: Arc::new(writer),
        }
    }

    pub fn id(&self) -> String {
        self.user.borrow().clone()
    }
}

impl DsotCore {
    pub async fn load_user(&self, user: &str, pass: Option<String>) -> Result<()> {
        let id = self.repo.users.load_user(user, pass).await?;
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
