use dioxus::prelude::*;

use crate::state::inbox::{InboxStateStoreExt, InboxStore, use_sync_inbox};
use crate::widgets::views::View;

mod header;
mod item;

static CSS: Asset = asset!("/assets/styles/view/inbox.css");

#[component]
pub fn InboxView() -> Element {
    let refresh = use_sync_inbox();

    let state = use_context::<InboxStore>();

    rsx! {
        View {
            name: "inbox",
            css: CSS,

            header::InboxHeader {
                refresh: refresh
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
}
