use async_trait::async_trait;

use crate::{error::Result, repository::UserRepository};

#[derive(Debug)]
pub struct UserNoopRepository {
    user: String,
}

impl UserNoopRepository {
    pub fn new(user: String) -> Self {
        Self { user }
    }
}

#[async_trait]
impl UserRepository for UserNoopRepository {
    async fn load_user(&self, _user: &str, _pass: Option<String>) -> Result<String> {
        Ok(self.user.clone())
    }
    async fn list_users(&self) -> Result<Vec<String>> {
        Ok(vec![self.user.clone()])
    }
}
