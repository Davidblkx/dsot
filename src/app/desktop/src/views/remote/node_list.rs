use dioxus::prelude::*;

use super::{add_address_button::AddAddressButton, node_menu_item::NodeMenuItem};
use crate::state::remote::{RemoteStateStoreExt, RemoteStore};

#[component]
pub fn NodeList(trigger: Signal<i32>) -> Element {
    let entries = use_context::<RemoteStore>().items();

    let index_list = use_memo(move || {
        entries
            .read()
            .iter()
            .enumerate()
            .map(|(i, _)| i)
            .collect::<Vec<usize>>()
    });

    rsx! {
        div {
            "data-component": "node_list",
            div {
                class: "actions",
                AddAddressButton { trigger }
            }
            div {
                class: "items",
                for i in index_list.iter() {
                    NodeMenuItem {
                        index: *i,
                        trigger
                    }
                }
            }
        }
    }
}
