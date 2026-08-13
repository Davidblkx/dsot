use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdPlus;
use dsot_shared_ui::{
    components::Modal,
    widgets::{icon::icon, inbox::FormAddInboxItem},
};

// TODO: redo this, not good enought design
#[component]
pub fn AddNewInboxItem() -> Element {
    let btn_icon = icon(LdPlus);
    let show_add_form = use_signal(|| false);

    rsx! {
        Modal {
            button_content: btn_icon,
            is_open: show_add_form,
            FormAddInboxItem {

            }
        }
    }
}
