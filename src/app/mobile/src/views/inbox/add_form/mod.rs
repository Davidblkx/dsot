mod base_form;
mod form_inputs;
mod inputs;
mod model;
mod type_selector;

use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdPlus;
use dsot_lib::{DsotCore, dsot_model::InboxValue, state::inbox::InboxOperations};
use dsot_shared_ui::widgets::icon::icon;

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
    let btn_icon = icon(LdPlus);
    let mut show_add_form = use_signal(|| false);

    let mut state = use_context::<Store<AddInboxValueState>>();

    rsx! {
        button {
            onclick: move |_| show_add_form.set(!show_add_form()),
            class: "action-btn",
            {btn_icon}
        }

        if show_add_form() {
            div {
                class: "add-form-inline",
                style: "position: absolute; top: 60px; left: 0; right: 0; padding: 10px; background: var(--color-bg-main); border-bottom: 1px solid var(--color-border); z-index: 10;",
                h3 { "Add new item" }
                base_form::InboxValueForm {
                    state: state.clone()
                }
                div {
                    style: "display: flex; gap: 10px; justify-content: flex-end; margin-top: 10px;",
                    button {
                        class: "btn-secondary",
                        onclick: move |_| {
                            show_add_form.set(false);
                            state.write().reset();
                        },
                        "Cancel"
                    }
                    button {
                        class: "btn-primary",
                        onclick: move |_| {
                            let _ = add_item(state.peek().cloned());
                            state.write().reset();
                            show_add_form.set(false);
                        },
                        "Save"
                    }
                }
            }
        }
    }
}
