use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdPlus;
use dsot_shared_ui::{
    components::{Dialog, DialogContentType},
    widgets::icon::icon,
};

use super::add_form::AddInboxItemForm;

#[component]
pub fn AddNewInboxItem() -> Element {
    let btn_icon = icon(LdPlus);
    let mut show_add_form = use_signal(|| false);

    let content = DialogContentType::Custom(rsx! {
        AddInboxItemForm {

        }
    });

    rsx! {
        Dialog {
            title: "Add new item",
            content: content,
            is_open: show_add_form,
            on_cancel: move |_| show_add_form.set(false),
        }

        button {
            onclick: move |_| show_add_form.set(true),
            {btn_icon}
        }
    }
}
