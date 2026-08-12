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
    async fn set_inbox_status(&self, id: Uuid, status: InboxStatus) -> Result<()>;
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
        let filter = self.state.inbox.table.get_table_ref();
        let items = self.repo.load_inbox(&filter).await?;
        self.state.inbox.set_items(items);
        Ok(())
    }
    async fn set_inbox_status(&self, id: Uuid, status: InboxStatus) -> Result<()> {
        if self.repo.update_inbox_status(id, status).await? {
            self.reload_inbox_items().await?;
        }
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
