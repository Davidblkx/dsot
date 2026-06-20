use dioxus::prelude::*;

use crate::state::inbox::{InboxStateStoreExt, InboxStore, use_sync_inbox};

mod header;
mod item;

#[component]
pub fn InboxView() -> Element {
    let refresh = use_sync_inbox();

    let state = use_context::<InboxStore>();

    rsx! {
        div {
            "data-component": "view",
            "data-view": "inbox",

            header::InboxHeader {
                refresh: refresh
            }
        }

        div {
            class: "items",
            for i in state.items().read().iter() {
                item::InboxItem {
                    item: i.clone()
                }
            }
        }
    }
}
