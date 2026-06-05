use dioxus::prelude::*;
use dsot_lib::{
    dsot_db_sync::{self, IntoSyncEntity},
    dsot_model::{InboxItem, InboxItemSqlRepository, InboxValue},
};

async fn db_insert_item(
    db_manager: &dsot_db_sync::DatabaseManager,
    value: &str,
) -> anyhow::Result<()> {
    if value.is_empty() {
        return anyhow::Ok(());
    }

    let item = InboxItem::new(InboxValue::Other(value.to_string()))?;
    let db = db_manager.open_database().await?;

    db.insert::<InboxItemSqlRepository>(&item.to_sync()).await?;

    db.close().await?;

    anyhow::Ok(())
}

#[derive(PartialEq, Clone, Props)]
pub struct InboxAddProps {
    on_add: EventHandler<()>,
}

#[component]
pub fn InboxAdd(props: InboxAddProps) -> Element {
    let state = use_context::<dsot_lib::DsotState>();
    let mut value = use_signal(|| "".to_string());

    let mut insert_item = use_action(move |txt: String| {
        let state = state.clone();
        async move {
            match db_insert_item(&state.db, txt.as_str()).await {
                Ok(_) => {
                    log::debug!("Inbox item inserted: {}", txt);
                    value.set("".to_string());
                    props.on_add.call(());
                    anyhow::Ok(())
                }
                Err(e) => {
                    log::warn!("Fail to insert inbox item: {e}");
                    anyhow::Ok(())
                }
            }
        }
    });

    rsx! {
        div {
            h3 { "Add Inbox Item" }
            div {
                input {
                    type: "text",
                    value: "{value}",
                    oninput: move |e| value.set(e.value())
                }
                button {
                    disabled: value.read().is_empty(),
                    onclick: move |_| {
                        insert_item.call(value.read().clone());
                    },
                    "Add Item"
                }
            }
        }
    }
}
