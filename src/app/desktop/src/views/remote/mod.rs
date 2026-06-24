use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdRouter};
use dsot_network::DsotNode;
use dsot_shared_ui::widgets::network::MyAddress;
use iroh::EndpointId;

use crate::widgets::views::{Header, View};

static CSS: Asset = asset!("/assets/styles/view/remote.css");

#[component]
pub fn RemoteView() -> Element {
    let mut address_str = use_signal(|| "".to_string());
    let mut id: Signal<Option<EndpointId>> = use_signal(|| None);
    let mut info: Signal<Option<DsotNode>> = use_signal(|| None);
    let missing_address = use_memo(move || address_str.read().is_empty());
    let missing_id = use_memo(move || !id.read().is_some());

    let mut validate = move || {
        let address_str = address_str.clone();
        let txt = address_str.read();
        match EndpointId::from_str(&txt) {
            Ok(pub_id) => id.set(Some(pub_id)),
            Err(_) => id.set(None),
        }
    };

    let icon = rsx! {
        Icon {
            icon: LdRouter
        }
    };

    rsx! {
        View {
            name: "remote",
            css: CSS,

            Header {
                title: "Remote",
                icon: icon,
                MyAddress {  }
            }

            input {
                value: address_str,
                oninput: move |e| address_str.set(e.value()),
            }

            button {
                disabled: missing_address,
                onclick: move |_| {
                    validate();
                },
                "Validate"
            }

            button {
                disabled: missing_id,
                onclick: move |_| {

                },
                "Connect"
            }
        }
    }
}
