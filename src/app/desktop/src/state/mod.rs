use dioxus::prelude::*;

pub mod inbox;
#[allow(dead_code)]
pub mod remote;
#[allow(dead_code)]
pub mod remote_nodes;

use inbox::{InboxState, InboxStore};
use remote::{RemoteState, RemoteStore};
use remote_nodes::use_remote_nodes_provider;

#[component]
pub fn AppStateProvier(children: Element) -> Element {
    use_context_provider::<InboxStore>(|| Store::new(InboxState::default()));
    use_context_provider::<RemoteStore>(|| Store::new(RemoteState::default()));
    use_remote_nodes_provider();

    rsx! {
        {children}
    }
}
