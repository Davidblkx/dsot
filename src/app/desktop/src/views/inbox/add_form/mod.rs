mod base_form;
mod form_inputs;
mod inputs;
mod model;
mod type_selector;

use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdPlus;
use dsot_shared_ui::{
    components::{Dialog, DialogContentType},
    widgets::icon::icon,
};

use model::AddInboxValueState;

#[component]
pub fn AddInboxValue() -> Element {
    let btn_icon = icon(LdPlus);
    let mut show_add_form = use_signal(|| false);

    let mut state = use_store(|| AddInboxValueState::default());

    let content = DialogContentType::Custom(rsx! {
        base_form::InboxValueForm {
            state: state.clone()
        }
    });

    rsx! {
        Dialog {
            title: "Add new item",
            content: content,
            is_open: show_add_form,
            on_cancel: move |_| {
                show_add_form.set(false);
                state.write().reset();
            }
        }

        button {
            onclick: move |_| show_add_form.set(true),
            {btn_icon}
        }
    }
}
