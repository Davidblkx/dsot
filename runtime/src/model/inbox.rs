use dsot_core::model::{
    JournalOperation,
    entities::inbox::{Inbox, sql::InboxSql},
};
use thiserror::Error;
use uuid::Uuid;

use crate::Runtime;
use crate::error::Result;

#[derive(Error, Debug)]
pub enum InboxError {
    #[error("Inbox item with id {0}, not found")]
    InboxNotFound(Uuid),
}

impl InboxError {
    pub fn to_err<T>(self) -> Result<T> {
        Err(self.into())
    }
}

pub trait InboxItems {
    fn count_inbox(&self) -> impl Future<Output = Result<i64>>;
    fn list_inbox(&self, length: i64, skip: i64) -> impl Future<Output = Result<Vec<Inbox>>>;
    fn add_inbox(&self, value: InboxInsertValue) -> impl Future<Output = Result<Uuid>>;
    fn update_inbox(&self, value: Inbox) -> impl Future<Output = Result<bool>>;
    fn delete_inbox(&self, id: Uuid) -> impl Future<Output = Result<bool>>;
}

impl InboxItems for Runtime {
    async fn count_inbox(&self) -> Result<i64> {
        let db = self.db.get_lib_db()?;
        let trx = db.create_db_transaction().await?;
        let (_, count) = InboxSql::count(trx).await?;
        Ok(count)
    }

    async fn list_inbox(&self, length: i64, skip: i64) -> Result<Vec<Inbox>> {
        let db = self.db.get_lib_db()?;
        let trx = db.create_db_transaction().await?;
        let (_, inbox_items) = InboxSql::list(trx, length, skip).await?;
        Ok(inbox_items)
    }

    async fn add_inbox(&self, value: InboxInsertValue) -> Result<Uuid> {
        let inbox: Inbox = value.into();
        let insert_op = inbox.sql_operation().create()?;

        log::trace!("Inserting new inbox item");
        let db = self.db.get_lib_db()?;
        db.create_and_apply(JournalOperation::SQL(insert_op))
            .await?;

        Ok(inbox.id)
    }

    async fn delete_inbox(&self, id: Uuid) -> Result<bool> {
        let db = self.db.get_lib_db()?;
        let trx = db.create_db_transaction().await?;

        let (_, curr) = InboxSql::fetch_by_id(trx, &id).await?;
        let curr = match curr {
            Some(c) => c,
            None => return Ok(false),
        };
        let delete_op = curr.sql_operation().delete();
        db.create_and_apply(JournalOperation::SQL(delete_op))
            .await?;
        Ok(true)
    }

    async fn update_inbox(&self, value: Inbox) -> Result<bool> {
        let mut ops = Vec::with_capacity(5);
        let db = self.db.get_lib_db()?;
        let trx = db.create_db_transaction().await?;

        let (_, curr) = InboxSql::fetch_by_id(trx, &value.id).await?;
        let curr = curr.ok_or(InboxError::InboxNotFound(value.id))?;

        if curr.title != value.title {
            let op = curr.sql_operation().update_title(value.title.clone())?;
            ops.push(op);
        }
        if curr.artist != value.artist {
            let op = curr.sql_operation().update_artist(value.artist.clone())?;
            ops.push(op);
        }
        if curr.album != value.album {
            let op = curr.sql_operation().update_album(value.album.clone())?;
            ops.push(op);
        }
        if curr.file != value.file {
            let op = curr.sql_operation().update_file(value.file.clone())?;
            ops.push(op);
        }
        if curr.extra_info != value.extra_info {
            let op = curr
                .sql_operation()
                .update_extra_info(value.extra_info.clone())?;
            ops.push(op);
        }

        let is_update = !ops.is_empty();
        for op in ops {
            log::trace!("Applying inbox update operation: {:?}", op);
            db.create_and_apply(JournalOperation::SQL(op)).await?;
        }

        Ok(is_update)
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct InboxInsertValue {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub file: Option<String>,
    pub extra_info: Option<String>,
}

impl InboxInsertValue {
    pub fn new() -> Self {
        Self {
            album: None,
            artist: None,
            extra_info: None,
            file: None,
            title: None,
        }
    }

    pub fn set_album(&mut self, value: impl Into<String>) {
        self.album = Some(value.into());
    }

    pub fn set_artist(&mut self, value: impl Into<String>) {
        self.artist = Some(value.into());
    }

    pub fn set_extra_info(&mut self, value: impl Into<String>) {
        self.extra_info = Some(value.into());
    }

    pub fn set_file(&mut self, value: impl Into<String>) {
        self.file = Some(value.into());
    }

    pub fn set_title(&mut self, value: impl Into<String>) {
        self.title = Some(value.into());
    }

    pub fn has_value(&self) -> bool {
        self.album.is_some() || self.artist.is_some() || self.file.is_some() || self.title.is_some()
    }
}

impl Into<Inbox> for InboxInsertValue {
    fn into(self) -> Inbox {
        Inbox {
            id: Uuid::now_v7(),
            album: self.album,
            artist: self.artist,
            extra_info: self.extra_info,
            file: self.file,
            title: self.title,
        }
    }
}
