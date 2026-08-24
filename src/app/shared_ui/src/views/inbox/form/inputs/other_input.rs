use dioxus::prelude::*;

use crate::views::inbox::form::{model::InboxFormStateStoreExt, state::InboxFormStore};

#[component]
pub fn OtherInput() -> Element {
    let store = use_context::<InboxFormStore>();

    rsx! {
        div {
            class: "form-group",
            label { "Task" }
            textarea {
                value: "{store.text()}",
                placeholder: "Enter context here...",
                onchange: move |evt| store.text().set(evt.value())
            }
        }
    }
}
