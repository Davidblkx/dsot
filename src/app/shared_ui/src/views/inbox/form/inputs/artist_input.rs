use dioxus::prelude::*;

use crate::views::inbox::form::{model::InboxFormStateStoreExt, state::InboxFormStore};

#[component]
pub fn ArtistInput() -> Element {
    let store = use_context::<InboxFormStore>();

    rsx! {
        div {
            class: "form-group",
            label { "Artist" }
            input {
                type: "text",
                placeholder: "Enter artist name...",
                value: "{store.text()}",
                onchange: move |evt| store.text().set(evt.value())
            }
        }
    }
}
