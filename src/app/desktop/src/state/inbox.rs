use dioxus::prelude::*;
use dsot_lib::{
    DsotState,
    dsot_db_sync::DatabaseManager,
    dsot_model::{InboxItem, InboxItemSqlRepository, InboxStatus, InboxValue},
    uuid::Uuid,
};

#[derive(Debug, Clone, PartialEq)]
pub struct InboxViewItem {
    pub id: Uuid,
    pub value: InboxValue,
    pub status: InboxStatus,
}

impl TryFrom<InboxItem> for InboxViewItem {
    type Error = anyhow::Error;

    fn try_from(value: InboxItem) -> anyhow::Result<InboxViewItem> {
        Ok(InboxViewItem {
            id: value.id,
            status: value.status.clone(),
            value: value.value()?,
        })
    }
}

#[derive(Store, Debug, Clone)]
pub struct InboxState {
    pub items: Vec<InboxViewItem>,
    pub size: i64,
    pub offset: i64,
}

impl Default for InboxState {
    fn default() -> Self {
        Self {
            items: vec![],
            size: 100,
            offset: 0,
        }
    }
}

pub type InboxStore = Store<InboxState>;

async fn fetch_items(
    db: &DatabaseManager,
    size: i64,
    offset: i64,
) -> anyhow::Result<Vec<InboxViewItem>> {
    let db = db.open_database().await?;
    let sql_items = db
        .list::<InboxItemSqlRepository>(size, offset)
        .await?
        .into_iter()
        .map(|i| i.into())
        .collect::<Vec<InboxItem>>();

    let mut items = Vec::new();
    for i in sql_items {
        match i.try_into() {
            Ok(item) => items.push(item),
            Err(e) => {
                ::log::debug!("{}", e);
            }
        }
    }
    Ok(items)
}

pub fn use_sync_inbox() -> Signal<i32> {
    let db = use_context::<DsotState>().db;
    let state = use_context::<InboxStore>();

    let manual_refresh = use_signal(|| 0);

    use_effect(move || {
        let current_offset = *state.offset().read();
        let current_size = *state.size().read();
        manual_refresh.read(); // Re-runs when we manually increment it
        let db = db.clone();

        spawn(async move {
            let inner_state = state.clone();

            match fetch_items(&db, current_size, current_offset).await {
                Ok(items) => {
                    *inner_state.items().write() = items;
                }
                Err(e) => ::log::error!("Failed to refresh inbox: {:?}", e),
            };
        });
    });

    // Return the trigger handle to the component
    manual_refresh
}

async fn insert_item(db: &DatabaseManager, item: InboxItem) -> anyhow::Result<()> {
    let db = db.open_database().await?;
    db.insert::<InboxItemSqlRepository>(&item.into()).await?;

    Ok(())
}

pub fn use_insert_inbox(mut refresh: Signal<i32>) -> impl Fn(InboxItem) {
    let db = use_context::<DsotState>().db;

    move |item: InboxItem| {
        let db = db.clone();
        spawn(async move {
            match insert_item(&db, item).await {
                Ok(_) => {
                    // 🚀 SUCCESS! Increment the signal.
                    // This instantly tells `use_sync_inbox` to fetch the new list!
                    refresh += 1;
                }
                Err(e) => ::log::error!("Failed to insert item: {:?}", e),
            };
        });
    }
}
