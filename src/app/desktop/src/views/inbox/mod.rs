use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdPlus};
use dsot_shared_ui::{components::Modal, widgets::inbox::FormAddInboxItem};

#[component]
pub fn InboxView() -> Element {
    let mut form_is_open = use_signal(|| false);

    rsx! {
        div {
            "data-component": "view",
            "data-view": "inbox",

            header {
                h1 {
                    "Inbox"
                }
                Modal {
                    button_content: rsx! {
                        Icon {
                            icon: LdPlus
                        }
                    },
                    is_open: form_is_open,
                    FormAddInboxItem {
                        on_save: move |_| {
                            form_is_open.toggle();
                        }
                    }
                }
            }
        }
    }
}
