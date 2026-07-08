use super::RemoteNode;
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum SelectedNode {
    Node(usize),
    #[default]
    None,
}

#[derive(Debug, Clone, PartialEq, Default, Store)]
pub struct RemoteNodesState {
    pub nodes: Vec<RemoteNode>,
    pub selected: SelectedNode,
}

pub type RemoteNodesStore = Store<RemoteNodesState>;

pub fn use_remote_nodes_context() -> RemoteNodesStore {
    use_context::<RemoteNodesStore>()
}

pub fn use_remote_nodes_provider() -> RemoteNodesStore {
    use_context_provider::<RemoteNodesStore>(|| Store::new(RemoteNodesState::default()))
}
