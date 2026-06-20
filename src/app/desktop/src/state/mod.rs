use dioxus::prelude::*;

pub mod inbox;

use inbox::{InboxState, InboxStore};

#[component]
pub fn AppStateProvier(children: Element) -> Element {
    use_context_provider::<InboxStore>(|| Store::new(InboxState::default()));

    rsx! {
        {children}
    }
}
