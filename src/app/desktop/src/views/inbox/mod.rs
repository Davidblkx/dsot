use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdInbox};
use dsot_lib::DsotCore;
use dsot_shared_ui::sink::*;

use crate::widgets::views::{Header, View};

mod filter;

static CSS: Asset = asset!("/assets/styles/view/inbox.css");

#[component]
pub fn InboxView() -> Element {
    let core = use_context::<DsotCore>();
    let some_filter = use_receiver(core.state.inbox.table.filter.clone());

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
                "{some_filter}"
            }
        }
    }
}
