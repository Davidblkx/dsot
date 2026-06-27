use dioxus::prelude::*;

pub mod inbox;
pub mod remote;

use inbox::{InboxState, InboxStore};
use remote::{RemoteState, RemoteStore};

#[component]
pub fn AppStateProvier(children: Element) -> Element {
    use_context_provider::<InboxStore>(|| Store::new(InboxState::default()));
    use_context_provider::<RemoteStore>(|| Store::new(RemoteState::default()));

    rsx! {
        {children}
    }
}
