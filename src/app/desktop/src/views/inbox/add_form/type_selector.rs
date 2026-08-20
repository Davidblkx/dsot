use dioxus::prelude::*;

use super::model::{AddInboxValueState, AddInboxValueStateStoreExt, InboxItemType};

#[component]
pub fn TypeSelector(state: Store<AddInboxValueState>) -> Element {
    rsx! {
        div {
            class: "type_selector",
            TypeEntry { item: InboxItemType::File, state }
            TypeEntry { item: InboxItemType::Artist, state }
            TypeEntry { item: InboxItemType::Album, state }
            TypeEntry { item: InboxItemType::Link, state }
            TypeEntry { item: InboxItemType::Other, state }
        }
    }
}

#[component]
pub fn TypeEntry(item: InboxItemType, mut state: Store<AddInboxValueState>) -> Element {
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
