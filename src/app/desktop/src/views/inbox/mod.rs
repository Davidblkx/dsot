use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdInbox};
use dsot_lib::DsotCore;
use dsot_shared_ui::sink::*;
use dsot_shared_ui::views::inbox::INBOX_CSS;

use crate::widgets::views::{Header, View};

pub mod add_form;
mod filter;
mod item;

const CSS: Asset = asset!("/assets/styles/view/inbox.css");
const COMBINED_CSS: &[Asset] = &[CSS, INBOX_CSS];

#[component]
pub fn InboxView() -> Element {
    let core = use_context::<DsotCore>();
    let items = use_receiver(core.state.inbox.items.clone());

    rsx! {
        View {
            name: "inbox",
            css: COMBINED_CSS,
            Header {
                title: "Inbox",

                icon: rsx! {
                    Icon {
                        icon: LdInbox
                    }
                },
                div {
                    class: "actions",
                    filter::InboxFilter {  }
                    add_form::AddInboxValue {  }
                }
            }


            div {
                class: "item-list",
                for i in items.read().iter() {
                    item::InboxItem {
                        key: "{i.id}",
                        item: i.clone()
                    }
                }
            }
        }
    }
}
