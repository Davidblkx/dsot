use async_trait::async_trait;

use crate::error::Result;

#[async_trait]
pub trait UserRepository: Send + Sync + std::fmt::Debug {
    async fn load_user(&self, user: &str, pass: Option<String>) -> Result<String>;
    async fn list_users(&self) -> Result<Vec<String>>;
}
