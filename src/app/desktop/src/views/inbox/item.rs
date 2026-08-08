use dioxus::prelude::*;
use dsot_lib::state::inbox::InboxItemValue;

#[component]
pub fn InboxItem(item: InboxItemValue) -> Element {
    rsx! {
        "{item.id}"
    }
}
