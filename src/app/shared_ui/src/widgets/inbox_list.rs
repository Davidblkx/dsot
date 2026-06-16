use dioxus::prelude::*;
use dsot_lib::{
    DsotState,
    dsot_db_sync::DatabaseManager,
    dsot_model::{InboxItem, InboxItemSql, InboxItemSqlRepository, InboxValue},
    uuid::Uuid,
};

static FORM_CSS: Asset = asset!("/assets/styles/widgets/inbox.css");

#[component]
pub fn InboxList() -> Element {
    let state = use_context::<DsotState>();
    let mut items = use_store(|| Vec::new());
    let mut update_items = use_action(move || {
        let state = state.clone();
        async move {
            match DisplayItem::load_inbox(&state.db).await {
                Ok(values) => {
                    items.set(values);
                    anyhow::Ok(())
                }
                Err(e) => {
                    log::warn!("Failed to load inbox: {}", e);
                    anyhow::Ok(())
                }
            }
        }
    });

    use_hook(|| {
        spawn(async move {
            let _ = update_items.call().await;
        });
    });

    let mut is_open = use_signal(|| false);

    rsx! {
        document::Link { rel: "stylesheet", href: FORM_CSS }
        div {
            crate::components::Modal {
                is_open: is_open,
                super::inbox::FormAddInboxItem {
                    on_save: move |_| {
                        is_open.set(false);
                        update_items.call();
                    }
                }
            }
            ul {
                for itm in items.iter() {
                    li {
                        key: "{itm.id()}",
                        "{itm.text()}"
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Store)]
struct DisplayItem {
    id: Uuid,
    text: String,
}

impl TryFrom<&InboxItemSql> for DisplayItem {
    type Error = dsot_lib::dsot_db_sync::DBSyncError;

    fn try_from(v: &InboxItemSql) -> std::prelude::v1::Result<Self, Self::Error> {
        let value: InboxItem = v.clone().into();

        Ok(Self {
            id: value.id,
            text: match value.value()? {
                InboxValue::File(path) => path,
                InboxValue::Artist(artist) => artist,
                InboxValue::Album { album, .. } => album,
                InboxValue::Link(link) => link,
                InboxValue::Other(text) => text,
            },
        })
    }
}

impl DisplayItem {
    pub async fn load_inbox(db_manager: &DatabaseManager) -> anyhow::Result<Vec<Self>> {
        let db = db_manager.open_database().await?;

        let sql_items = db.list::<InboxItemSqlRepository>(100, 0).await?;

        let mut items = Vec::new();

        for i in sql_items.iter() {
            match i.try_into() {
                Ok(display_item) => items.push(display_item),
                Err(e) => {
                    log::warn!("Failed to convert inbox item: {}", e);
                }
            }
        }

        db.close().await?;

        Ok(items)
    }
}
