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
    Waiting,
    Pending,
    Done,
    Syncing,
    Failure,
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
