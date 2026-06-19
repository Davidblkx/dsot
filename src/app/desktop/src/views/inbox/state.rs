use dioxus::prelude::*;
use dsot_lib::{
    DsotState,
    dsot_model::{InboxItem, InboxItemSql, InboxItemSqlRepository},
};

#[derive(Store, Debug, Clone)]
pub struct InboxStore {
    pub items: Vec<InboxItem>,
}

impl InboxStore {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub async fn refresh(&mut self, dsot: &DsotState) -> anyhow::Result<()> {
        let db = dsot.db.open_database().await?;
        let items = db.list::<InboxItemSqlRepository>(1000, 0).await?;
        self.items = items
            .into_iter()
            .map(|i| i.into())
            .collect::<Vec<InboxItem>>();

        Ok(())
    }

    pub async fn insert(&mut self, dsot: &DsotState, item: &InboxItemSql) -> anyhow::Result<()> {
        let db = dsot.db.open_database().await?;
        db.insert::<InboxItemSqlRepository>(item).await?;

        self.refresh(dsot).await?;

        Ok(())
    }
}
