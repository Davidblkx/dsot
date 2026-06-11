use dioxus::prelude::*;

use crate::widgets::InboxList;

#[component]
pub fn InboxView() -> Element {
    rsx! {
        h1 { "Inbox" }
        InboxList {  }
    }
}
