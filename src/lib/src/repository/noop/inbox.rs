use async_trait::async_trait;

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
}
