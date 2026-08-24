use dioxus::prelude::*;

use crate::views::inbox::form::{model::InboxFormStateStoreExt, state::InboxFormStore};

#[component]
pub fn FileInput() -> Element {
    let store = use_context::<InboxFormStore>();

    rsx! {
        div {
            class: "form-group",
            label {
                "File to import {store.text()}"
            }
            input {
                type: "file",
                multiple: false,
                onchange: move |ev| {
                    store.text().set(ev.value());
                }
            }

        }
    }
}
