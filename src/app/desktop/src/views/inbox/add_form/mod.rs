mod base_form;
mod form_inputs;
mod inputs;
mod model;
mod type_selector;

use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdPlus;
use dsot_lib::{DsotCore, dsot_model::InboxValue, state::inbox::InboxOperations};
use dsot_shared_ui::{
    components::{Dialog, DialogContentType},
    widgets::icon::icon,
};

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

#[component]
pub fn AddInboxValue() -> Element {
    let add_item = use_add_item();
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
            },
            on_ok: move |_| {
                let _ = add_item(state.peek().cloned());
                state.write().reset();
                show_add_form.set(false);
            }
        }

        button {
            onclick: move |_| show_add_form.set(true),
            {btn_icon}
        }
    }
}
