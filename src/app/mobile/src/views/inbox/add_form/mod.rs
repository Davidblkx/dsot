mod base_form;
mod form_inputs;
mod inputs;
mod model;
mod type_selector;

use dioxus::prelude::*;
use dsot_lib::{DsotCore, dsot_model::InboxValue, state::inbox::InboxOperations};

use model::AddInboxValueState;



pub fn use_provide_add_inbox_state() {
    let state = use_store(|| AddInboxValueState::default());
    use_context_provider(|| state);
}

#[component]
pub fn AddInboxValue() -> Element {
    let dsot = use_context::<DsotCore>();
    let mut state = use_context::<Store<AddInboxValueState>>();
    let navigator = use_navigator();

    rsx! {
        div {
            class: "add-form-page",
            style: "padding: 20px; display: flex; flex-direction: column; height: 100%; gap: 20px; overflow-y: auto;",
            h1 { "Add new item" }
            base_form::InboxValueForm {
                state: state.clone()
            }
            div {
                style: "display: flex; gap: 15px; justify-content: flex-end; margin-top: auto; padding-top: 20px;",
                button {
                    class: "btn-secondary",
                    style: "padding: 10px 20px; font-size: 1.1rem;",
                    onclick: move |_| {
                        state.write().reset();
                        navigator.push(crate::routes::Routes::InboxView);
                    },
                    "Cancel"
                }
                button {
                    class: "btn-primary",
                    style: "padding: 10px 20px; font-size: 1.1rem; background-color: var(--color-primary); color: var(--color-bg-main);",
                    onclick: move |_| {
                        let dsot = dsot.clone();
                        async move {
                            let item: InboxValue = state.peek().cloned().into();
                            match dsot.add_inbox_item(item).await {
                                Ok(_) => {
                                    state.write().reset();
                                    navigator.push(crate::routes::Routes::InboxView);
                                }
                                Err(err) => log::error!("Failed to add inbox item: {}", err),
                            }
                        }
                    },
                    "Save"
                }
            }
        }
    }
}
