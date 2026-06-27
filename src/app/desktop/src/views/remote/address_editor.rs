use dioxus::prelude::*;

use crate::state::remote::NewNetworkAddress;

#[component]
pub fn AddressEditor(mut address: Signal<NewNetworkAddress>) -> Element {
    rsx! {
        form {
            "data-component": "address_editor",
            input {
                value: address.read().id.clone(),
                oninput: move |evt| {
                    address.write().id = evt.value()
                }
            },
        }
    }
}
