use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdInbox};

use crate::widgets::views::{Header, View};

mod filter;

static CSS: Asset = asset!("/assets/styles/view/inbox.css");

#[component]
pub fn InboxView() -> Element {
    rsx! {
        View {
            name: "inbox",
            css: CSS,
            Header {
                title: "Inbox",
                icon: rsx! {
                    Icon {
                        icon: LdInbox
                    }
                },
                filter::InboxFilter {  }
            }


            div {
                "Inbox Content"
            }
        }
    }
}
