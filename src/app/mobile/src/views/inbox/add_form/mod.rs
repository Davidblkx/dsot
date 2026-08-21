mod base_form;
mod form_inputs;
mod inputs;
mod model;
mod type_selector;

use dioxus::prelude::*;
use dsot_lib::{DsotCore, dsot_model::InboxValue, state::inbox::InboxOperations};

use model::AddInboxValueState;

fn use_add_item() -> impl Fn(AddInboxValueState) -> bool {
    let dsot = use_context::<DsotCore>();

    move |state: AddInboxValueState| {
        let dsot = dsot.clone();
        let item: InboxValue = state.into();

        spawn(async move {
            match dsot.add_inbox_item(item).await {
                Ok(_) => {}
                Err(err) => log::error!("Failed to add inbox item: {}", err),
            }
        });

        return true;
    }
}

pub fn use_provide_add_inbox_state() {
    let state = use_store(|| AddInboxValueState::default());
    use_context_provider(|| state);
}

#[component]
pub fn AddInboxValue() -> Element {
    let add_item = use_add_item();
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
                        let _ = add_item(state.peek().cloned());
                        state.write().reset();
                        navigator.push(crate::routes::Routes::InboxView);
                    },
                    "Save"
                }
            }
        }
    }
}
