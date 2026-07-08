use dioxus::prelude::*;
use dsot_lib::DsotState;
use dsot_network::NetworkAddress;

use super::{RemoteNode, RemoteNodesStateStoreExt, SelectedNode, use_remote_nodes_context};

/// Loads the initial address book and return trigger signal to reload it
pub fn use_address_book_reader() -> Signal<i32> {
    let dsot = use_context::<DsotState>();
    let nodes_store = use_remote_nodes_context();
    let trigger = use_signal(|| 0);

    use_effect(move || {
        let dsot = dsot.clone();
        trigger.read();
        if let Some(net) = dsot.network {
            let items = net
                .address_book
                .read_safe()
                .addresses
                .into_iter()
                .map(|i| i.into())
                .collect::<Vec<RemoteNode>>();

            nodes_store.nodes().set(items);
            nodes_store.selected().set(SelectedNode::None);
        }
    });

    trigger
}

/// Writes the current node list to address book
pub fn use_address_book_writer() -> impl Fn() -> bool {
    let dsot = use_context::<DsotState>();
    let nodes_store = use_remote_nodes_context();

    move || {
        let dsot = dsot.clone();
        if let Some(net) = dsot.network {
            let address_list: Vec<NetworkAddress> = nodes_store
                .nodes()
                .read()
                .clone()
                .into_iter()
                .map(|e| e.address)
                .collect();
            match net.address_book.write_addresses(address_list) {
                Ok(_) => true,
                Err(e) => {
                    ::log::error!("Failed to write address book: {}", e);
                    false
                }
            }
        } else {
            false
        }
    }
}
