use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdInbox, LdPlus},
};
use dsot_shared_ui::{components::Modal, widgets::inbox::FormAddInboxItem};

use crate::state::inbox::use_insert_inbox;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct HeaderProps {
    refresh: Signal<i32>,
}

#[component]
pub fn InboxHeader(props: HeaderProps) -> Element {
    let insert_inbox = use_insert_inbox(props.refresh);
    let mut form_is_open = use_signal(|| false);

    rsx! {
        header {
            span {
                class: "icon",
                Icon {
                    icon: LdInbox
                }
            }
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
}
