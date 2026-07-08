use dioxus::prelude::*;

use crate::state::remote_nodes::{RemoteNode, RemoteNodesStateStoreExt, use_remote_nodes_context};

pub fn use_node_by_index(index: usize) -> RemoteNode {
    let ctx = use_remote_nodes_context().nodes();

    ctx.read().get(index).cloned().unwrap_or_default()
}
