use std::str::FromStr;

use dioxus::prelude::*;
use dsot_lib::DsotState;
use iroh::PublicKey;

use crate::state::remote::NewNetworkAddress;

#[component]
pub fn AddressEditor(mut address: Signal<NewNetworkAddress>) -> Element {
    let state = use_context::<DsotState>();

    let mut fetch_info = move |mut address: Signal<NewNetworkAddress>| {
        if let Some(network) = state.network.clone() {
            if let Ok(id) = PublicKey::from_str(&address.read().id) {
                spawn(async move {
                    match network.connect_node(id).await {
                        Ok(node) => {
                            let mut writer = address.write();
                            writer.name = node.info.name;
                            writer.desc = node.info.desc;
                        }
                        Err(e) => {
                            ::log::warn!("Failed to connect to node: {}", e);
                        }
                    }
                });
            } else {
                ::log::warn!("Invalid public key: {}", address.read().id);
            }
        }
    };

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
                            fetch_info(address.clone());
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
