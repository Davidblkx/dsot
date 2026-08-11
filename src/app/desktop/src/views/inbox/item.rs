use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdAirVent};
use dsot_lib::state::inbox::InboxItemValue;

#[component]
pub fn InboxItem(item: InboxItemValue) -> Element {
    let content = match item.value {
        _ => rsx! {
            Icon {
                icon: LdAirVent
            },
            div {
                class: "content",
                "Unknown"
            }
        },
    };

    rsx! {
        {content}
    }
}
