use dsot_core::model::{
    JournalOperation,
    entities::inbox::{Inbox, sql::InboxSql},
};
use uuid::Uuid;

use crate::Runtime;
use crate::error::Result;

pub trait InboxItems {
    fn count_inbox(&self) -> impl Future<Output = Result<i64>>;
    fn list_inbox(&self, length: i64, skip: i64) -> impl Future<Output = Result<Vec<Inbox>>>;
    fn add_inbox(&self, value: InboxInsertValue) -> impl Future<Output = Result<Uuid>>;
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
}

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
