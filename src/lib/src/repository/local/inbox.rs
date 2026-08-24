use async_trait::async_trait;
use dsot_db_sync::DBSyncError;
use dsot_model::{InboxItem, InboxItemSql, InboxItemSqlRepository, InboxStatus, InboxValue};
use uuid::Uuid;

use crate::{
    error::{DsotError, Result},
    repository::InboxRepository,
    sink::TableFilter,
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
        if !*self.user.is_logged_in().borrow() {
            return Ok(vec![]);
        }

        let db = self.user.open_db().await?;
        let mut res = Vec::new();

        let items = db
            .list::<InboxItemSqlRepository>(filter.size, filter.offset)
            .await?;

        for i in items.into_iter() {
            let item = i.try_into()?;

            if filter.filter.include(&item) {
                res.push(item);
            }
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
    async fn update_inbox_status(&self, id: Uuid, status: InboxStatus) -> Result<bool> {
        let db = self.user.open_db().await?;
        let mut item = match db.get::<InboxItemSqlRepository>(id).await {
            Ok(item) => item,
            Err(err) => match err {
                DBSyncError::EntityNotFound(_, _) => return Ok(false),
                e => return Err(e.into()),
            },
        };

        if item.status == status {
            return Ok(false);
        }

        item.status = status;
        db.update::<InboxItemSqlRepository>(&item).await?;

        Ok(true)
    }
    async fn get_inbox_item(&self, id: Uuid) -> Result<InboxItemValue> {
        let db = self.user.open_db().await?;
        db.get::<InboxItemSqlRepository>(id).await?.try_into()
    }
    async fn update_inbox_item(&self, id: Uuid, value: InboxValue) -> Result<()> {
        let db = self.user.open_db().await?;
        let mut item = match db.get::<InboxItemSqlRepository>(id).await {
            Ok(item) => item,
            Err(err) => match err {
                e => return Err(e.into()),
            },
        };

        item.set_value(value)?;
        db.update::<InboxItemSqlRepository>(&item).await?;

        Ok(())
    }
}

impl TryInto<InboxItemValue> for InboxItemSql {
    type Error = DsotError;

    fn try_into(self) -> std::prelude::v1::Result<InboxItemValue, Self::Error> {
        Ok(InboxItemValue {
            id: self.id,
            status: self.status,
            value: self.value()?,
        })
    }
}
