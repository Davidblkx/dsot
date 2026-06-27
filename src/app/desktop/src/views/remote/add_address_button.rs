use dioxus::prelude::*;

use dsot_shared_ui::components::{Dialog, DialogContentType};

use super::address_editor::AddressEditor;
use crate::state::remote::NewNetworkAddress;

#[component]
pub fn AddAddressButton(trigger: Signal<i32>) -> Element {
    let mut is_open = use_signal(|| false);
    let new_address = use_signal(|| NewNetworkAddress::default());

    let content = DialogContentType::Custom(rsx! {
        AddressEditor {
            address: new_address
        }
    });

    rsx! {
        button {
            onclick: move |_| {
                is_open.set(true);
            },
            "Add Remote Device"
        }
        Dialog {
            title: "Add Remote Device",
            is_open: is_open,
            content: content,
            on_cancel: move |_| {
                is_open.set(false);
            },
            on_ok: |_| {

            }
        }
    }
}
