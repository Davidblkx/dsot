mod state;

use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdPlus};
use dsot_lib::DsotState;
use dsot_shared_ui::{components::Modal, widgets::inbox::FormAddInboxItem};

pub use state::InboxStore;

#[component]
pub fn InboxView() -> Element {
    let dsot = use_context::<DsotState>();
    let mut store = use_context::<crate::helpers::view_context::ViewContext>().inbox;
    let mut form_is_open = use_signal(|| false);
    let items = store.read().items.clone();

    spawn(async move {
        match store.write().refresh(&dsot).await {
            Ok(_) => {}
            Err(e) => {
                log::error!("Failed to refresh inbox: {:?}", e);
            }
        }
    });

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

            div {
                class: "items",
                for itm in items  {
                    div {
                        class: "item",
                        "{itm.id}"
                    }
                }
            }
        }
    }
}
