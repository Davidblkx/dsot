use std::str::FromStr;
use dioxus::prelude::*;

use dsot_shared_ui::components::{Dialog, DialogContentType};

use super::address_editor::AddressEditor;
use crate::state::remote::{NewNetworkAddress, RemoteStore, RemoteMachine, MachineStatus, SyncStatus};

#[component]
pub fn AddAddressButton(trigger: Signal<i32>) -> Element {
    let mut is_open = use_signal(|| false);
    let mut new_address = use_signal(|| NewNetworkAddress::default());
    let state = use_context::<RemoteStore>();

    let content = DialogContentType::Custom(rsx! {
        AddressEditor {
            address: new_address
        }
    });

    rsx! {
        button {
            class: "btn-add-device",
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
                new_address.set(NewNetworkAddress::default());
            },
            on_ok: move |_| {
                let addr = new_address.read().clone();
                if let Ok(endpoint_id) = iroh::EndpointId::from_str(&addr.id) {
                    let mut state = state.clone();
                    let mut trigger = trigger.clone();
                    state.write().items.push(RemoteMachine {
                        id: endpoint_id,
                        name: addr.name,
                        desc: addr.desc,
                        status: MachineStatus::Offline,
                        sync: SyncStatus::Disabled,
                    });
                    *trigger.write() += 1;
                    is_open.set(false);
                    new_address.set(NewNetworkAddress::default());
                } else {
                    log::error!("Invalid Iroh Endpoint ID format: {}", addr.id);
                }
            }
        }
    }
}
