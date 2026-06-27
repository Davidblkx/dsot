use dioxus::stores::Store;
use dsot_network::DsotNode;

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
pub struct RemoteState {
    items: Vec<RemoteMachine>,
    selected: Option<RemoteMachine>,
}

pub type RemoteStore = Store<RemoteState>;
