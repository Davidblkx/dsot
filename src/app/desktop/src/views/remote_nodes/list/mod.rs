use dioxus::prelude::*;

use crate::state::remote_nodes::{RemoteNodesStateStoreExt, use_remote_nodes_context};

mod item;

#[component]
pub fn RemoteNodeList(trigger: Signal<i32>) -> Element {
    let nodes = use_remote_nodes_context().nodes();
    let list = use_memo(move || {
        nodes
            .read()
            .iter()
            .enumerate()
            .map(|(i, _)| i)
            .collect::<Vec<usize>>()
    });

    rsx! {
        div {
            class: "list_node",
            div {
                class: "actions"
            }
            div {
                class: "nodes",
                for index in list.iter() {
                    item::NodeListItem { index: *index }
                }
            }
        }
    }
}
