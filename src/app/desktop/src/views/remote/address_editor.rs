use dioxus::prelude::*;

use crate::state::remote::NewNetworkAddress;

#[component]
pub fn AddressEditor(mut address: Signal<NewNetworkAddress>) -> Element {
    rsx! {
        form {
            "data-component": "address_editor",
            class: "address-editor-form",
            
            div {
                class: "form-group",
                label {
                    "for": "device_id",
                    "Endpoint ID (Iroh Address)*"
                }
                div {
                    class: "input-group-row",
                    input {
                        id: "device_id",
                        placeholder: "Enter the cryptographically secure endpoint public key",
                        required: true,
                        value: address.read().id.clone(),
                        oninput: move |evt| {
                            address.write().id = evt.value();
                        }
                    }
                    button {
                        class: "btn-fetch-info",
                        r#type: "button",
                        disabled: address.read().id.trim().is_empty(),
                        onclick: move |_| {
                            log::info!("Fetching info for Endpoint ID: {}", address.read().id);
                        },
                        "Fetch Info"
                    }
                }
            }

            div {
                class: "form-group",
                label {
                    "for": "device_name",
                    "Device Name*"
                }
                input {
                    id: "device_name",
                    placeholder: "e.g. My Laptop, Living Room Pi",
                    required: true,
                    value: address.read().name.clone(),
                    oninput: move |evt| {
                        address.write().name = evt.value();
                    }
                }
            }

            div {
                class: "form-group",
                label {
                    "for": "device_desc",
                    "Description"
                }
                textarea {
                    id: "device_desc",
                    placeholder: "Optional notes about this device...",
                    value: address.read().desc.clone(),
                    oninput: move |evt| {
                        address.write().desc = evt.value();
                    }
                }
            }
        }
    }
}
