use async_trait::async_trait;
use dsot_model::InboxValue;
use uuid::Uuid;

use crate::{
    error::Result,
    repository::InboxRepository,
    state::inbox::{InboxFilter, InboxItemValue},
};

#[derive(Debug)]
pub struct NoopInboxRepository;

#[async_trait]
impl InboxRepository for NoopInboxRepository {
    async fn load_inbox(&self, _filter: &InboxFilter) -> Result<Vec<InboxItemValue>> {
        Ok(vec![])
    }
    async fn add_inbox_item(&self, _value: InboxValue) -> Result<()> {
        Ok(())
    }
    async fn remove_inbox_item(&self, _id: Uuid) -> Result<bool> {
        Ok(true)
    }
}
