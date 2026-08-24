use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdPlus;
use dsot_shared_ui::{
    components::{Dialog, DialogButtonType, DialogContentType},
    views::inbox::form::InboxForm,
    widgets::icon::icon,
};

#[component]
pub fn AddInboxValue() -> Element {
    let btn_icon = icon(LdPlus);
    let mut show_add_form = use_signal(|| false);

    let content = DialogContentType::Custom(rsx! {
        InboxForm {
            on_save: move |_| {
                show_add_form.set(false);
            }
        }
    });
    let buttons = DialogButtonType::Custom(rsx! {});

    rsx! {
        Dialog {
            title: "Add new item",
            content: content,
            is_open: show_add_form,
            on_cancel: move |_| {
                show_add_form.set(false);
            },
            buttons,
        }

        button {
            onclick: move |_| show_add_form.set(true),
            {btn_icon}
        }
    }
}
