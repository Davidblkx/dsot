use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdPlus};
use dsot_shared_ui::{components::Modal, widgets::inbox::FormAddInboxItem};

use crate::state::inbox::{InboxStateStoreExt, InboxStore, use_insert_inbox, use_sync_inbox};

#[component]
pub fn InboxView() -> Element {
    let refresh = use_sync_inbox();
    let insert_inbox = use_insert_inbox(refresh);

    let state = use_context::<InboxStore>();
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
                        on_save: move |item| {
                            insert_inbox(item);
                            form_is_open.toggle();
                        }
                    }
                }
            }
        }

        div {
            class: "items",
            for i in state.items().read().iter() {
                div {
                    class: "item",
                    "{i.id}"
                }
            }
        }
    }
}
