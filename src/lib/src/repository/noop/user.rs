use async_trait::async_trait;

use crate::{error::Result, repository::UserRepository};

#[derive(Debug)]
pub struct UserNoopRepository;

impl UserNoopRepository {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl UserRepository for UserNoopRepository {
    async fn list_users(&self) -> Result<Vec<String>> {
        Ok(vec![])
    }
}
