use crate::state::remote::MachineStatus;

use super::machine::RemoteMachine;
use super::store::{RemoteStateStoreExt, RemoteStore, SelectedMachine};
use dioxus::prelude::*;
use dsot_lib::DsotState;
use dsot_network::NetworkAddress;

pub fn use_remote_machines() -> Signal<i32> {
    let state = use_context::<DsotState>();
    let remote_state = use_context::<RemoteStore>();
    let manual_refresh = use_signal(|| 0);

    use_effect(move || {
        let inner_state = state.clone();
        manual_refresh.read(); // Re-render on manual refresh
        if let Some(network) = inner_state.network {
            let items = network
                .address_book
                .read_safe()
                .addresses
                .into_iter()
                .map(|i| i.into())
                .collect::<Vec<RemoteMachine>>();
            *remote_state.items().write() = items;
            remote_state.selected().set(SelectedMachine::None);
        }
    });

    manual_refresh
}

pub fn use_node_insert(trigger: Signal<i32>) -> impl Fn(RemoteMachine) {
    let dsot = use_context::<DsotState>();
    let devices = use_context::<RemoteStore>();

    move |item: RemoteMachine| {
        let dsot = dsot.clone();

        let devices = devices.clone();
        devices.items().write().push(item);

        let mut trigger = trigger.clone();

        if let Some(net) = dsot.network {
            let addresses: Vec<NetworkAddress> = devices
                .items()
                .read()
                .iter()
                .map(|e| e.clone().into())
                .collect();
            match net.address_book.write_addresses(addresses) {
                Ok(_) => trigger += 1,
                Err(e) => {
                    ::log::error!("Failed to write address book: {}", e)
                }
            }
        }
    }
}

pub fn use_remove_selected(trigger: Signal<i32>) -> impl Fn() {
    let dsot = use_context::<DsotState>();
    let devices = use_context::<RemoteStore>();

    move || {
        let dsot = dsot.clone();
        let mut trigger = trigger.clone();
        let index = match *devices.selected().read() {
            SelectedMachine::Machine(index) => index,
            SelectedMachine::None => return,
        };

        if let Some(net) = dsot.network {
            let mut addresses: Vec<NetworkAddress> = devices
                .items()
                .read()
                .iter()
                .map(|e| e.clone().into())
                .collect();

            if index < addresses.len() {
                addresses.remove(index);
                match net.address_book.write_addresses(addresses) {
                    Ok(_) => trigger += 1,
                    Err(e) => {
                        ::log::error!("Failed to write address book: {}", e)
                    }
                }
            }
        }
    }
}

pub fn use_node_connect(index: usize) {
    let dsot = use_context::<DsotState>();
    let state = use_context::<RemoteStore>();

    use_future(move || {
        let network_opt = dsot.network.clone();
        async move {
            if let Some(net) = network_opt {
                let id = state.items().read()[index].id.clone();

                match net.connect_node(id).await {
                    Ok(node) => {
                        state.items().write()[index].status = MachineStatus::Online(node);
                    }
                    Err(e) => {
                        ::log::error!("Failed to connect node: {}", e)
                    }
                }
            }
        }
    });
}
