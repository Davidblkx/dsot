use async_trait::async_trait;

use dsot_model::{InboxStatus, InboxValue};
use uuid::Uuid;

use super::InboxFilterValue;
use crate::{DsotCore, error::Result, repository::InboxRepository, sink::TableRefWatch};

#[async_trait]
pub trait InboxOperations {
    async fn add_inbox_item(&self, value: InboxValue) -> Result<()>;
    async fn remove_inbox_item(&self, id: Uuid) -> Result<bool>;
    async fn reload_inbox_items(&self) -> Result<()>;
}

#[async_trait]
impl InboxOperations for DsotCore {
    async fn add_inbox_item(&self, value: InboxValue) -> Result<()> {
        self.repo.add_inbox_item(value).await?;
        self.reload_inbox_items().await?;
        Ok(())
    }
    async fn remove_inbox_item(&self, id: Uuid) -> Result<bool> {
        if self.repo.remove_inbox_item(id).await? {
            self.reload_inbox_items().await?;
        }
        Ok(true)
    }
    async fn reload_inbox_items(&self) -> Result<()> {
        Ok(())
    }
}

pub trait InboxFilterOperations {
    fn update_status(&self, status: Option<InboxStatus>);
}

impl InboxFilterOperations for TableRefWatch<InboxFilterValue> {
    fn update_status(&self, status: Option<InboxStatus>) {
        self.mod_filter(|f| {
            if f.status == status {
                return None;
            }

            let mut f = f.clone();
            f.status = status;
            Some(f)
        });
    }
}
