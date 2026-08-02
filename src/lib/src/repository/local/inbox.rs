use async_trait::async_trait;
use dsot_model::{InboxItem, InboxItemSql, InboxItemSqlRepository, InboxValue};
use uuid::Uuid;

use crate::{
    error::Result,
    repository::InboxRepository,
    state::inbox::{InboxFilter, InboxItemValue},
    user::DsotUser,
};

#[derive(Debug)]
pub struct InboxLocalRepository {
    user: DsotUser,
}

impl InboxLocalRepository {
    pub fn new(user: DsotUser) -> Self {
        Self { user }
    }
}

#[async_trait]
impl InboxRepository for InboxLocalRepository {
    async fn load_inbox(&self, filter: &InboxFilter) -> Result<Vec<InboxItemValue>> {
        let db = self.user.open_db().await?;
        let mut res = Vec::new();

        let items = db
            .list::<InboxItemSqlRepository>(filter.size, filter.offset)
            .await?;

        for i in items.iter() {
            let id = i.id;
            let value = i.value()?;
            let status = i.status.clone();

            res.push(InboxItemValue { id, value, status });
        }

        Ok(res)
    }
    async fn add_inbox_item(&self, value: InboxValue) -> Result<()> {
        let item: InboxItemSql = InboxItem::new(value)?.into();
        let db = self.user.open_db().await?;
        db.insert::<InboxItemSqlRepository>(&item).await?;
        Ok(())
    }
    async fn remove_inbox_item(&self, id: Uuid) -> Result<bool> {
        let db = self.user.open_db().await?;
        db.delete::<InboxItemSqlRepository>(id).await?;
        Ok(true)
    }
}
