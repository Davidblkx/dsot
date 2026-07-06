use dioxus::prelude::*;
use dsot_lib::DsotState;
use dsot_network::{DsotNode, NetworkAddress};

#[derive(Debug, Clone, PartialEq)]
pub enum MachineStatus {
    Offline,
    Online(DsotNode),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyncStatus {
    Disabled,
    Pending,
    InSync,
    Failure,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct NewNetworkAddress {
    pub id: String,
    pub name: String,
    pub desc: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoteMachine {
    pub id: iroh::EndpointId,
    pub name: String,
    pub desc: String,
    pub status: MachineStatus,
    pub sync: SyncStatus,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum SelectedMachine {
    Machine(usize),
    #[default]
    None,
}

#[derive(Debug, Clone, PartialEq, Default, Store)]
pub struct RemoteState {
    pub items: Vec<RemoteMachine>,
    pub selected: SelectedMachine,
}

pub type RemoteStore = Store<RemoteState>;

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

impl From<NetworkAddress> for RemoteMachine {
    fn from(value: NetworkAddress) -> Self {
        Self {
            id: value.address,
            name: value.name,
            desc: value.desc,
            status: MachineStatus::Offline,
            sync: SyncStatus::Disabled,
        }
    }
}

impl Into<NetworkAddress> for RemoteMachine {
    fn into(self) -> NetworkAddress {
        NetworkAddress {
            address: self.id,
            name: self.name,
            desc: self.desc,
        }
    }
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
