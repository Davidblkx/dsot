use dioxus::prelude::*;

use crate::views::inbox::form::model::InboxFormStateStoreExt;

use super::{model::InboxItemType, state::InboxFormStore};

#[component]
pub fn InboxFormTypeSelector() -> Element {
    rsx! {
        nav {
            class: "type_selector",
            TypeEntry { item: InboxItemType::File }
            TypeEntry { item: InboxItemType::Artist }
            TypeEntry { item: InboxItemType::Album }
            TypeEntry { item: InboxItemType::Link }
            TypeEntry { item: InboxItemType::Other }
        }
    }
}

#[component]
pub fn TypeEntry(item: InboxItemType) -> Element {
    let state = use_context::<InboxFormStore>();
    let is_selected = item.get_name() == state.form_type()().get_name();
    let class = if is_selected {
        "item_type selected"
    } else {
        "item_type"
    };
    let (name, icon) = item.get_icon_name();

    rsx! {
        div {
            class: class,
            onclick: move |_|  {
                if is_selected {
                    return;
                }
                state.form_type().set(item.clone());
            },
            {icon},
            span {
                class: "text",
                "{name}"
            }
        }
    }
}
