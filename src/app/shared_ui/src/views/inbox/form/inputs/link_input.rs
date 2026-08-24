use dioxus::prelude::*;

use crate::views::inbox::form::{model::InboxFormStateStoreExt, state::InboxFormStore};

#[component]
pub fn LinkInput() -> Element {
    let store = use_context::<InboxFormStore>();

    rsx! {
        div {
            class: "form-group",
            label { "URL Link" }
            input {
                type: "text",
                placeholder: "https://example.com",
                value: "{store.text()}",
                onchange: move |evt| {
                    let mut val = evt.value();
                    if val.len() > 0 && !val.contains("://") {
                        val = format!("https://{}", val);
                    }
                    store.text().set(val);
                }
            }
        }
    }
}
