use dioxus::prelude::*;
use dsot_lib::DsotCore;

#[component]
pub fn AppAddress() -> Element {
    let core = use_context::<DsotCore>();

    let id = use_resource(move || {
        let net = core.net.clone();

        async move {
            match net.connect().await {
                Ok(e) => Some(e.id()),
                Err(err) => {
                    ::log::warn!("Error connecting to network: {}", err);
                    None
                }
            }
        }
    });

    rsx! {
        match &*id.read_unchecked() {
            Some(Some(addr)) => rsx! {
                span {
                    class: "net_address current",
                    "{addr}"
                }
            },
            Some(None) => rsx! {
                span {
                    class: "net_address offline",
                    "Device is offline"
                }
            },
            None => rsx! {
                span {
                    class: "net_address loading",
                    "Connecting..."
                }
            },
        }
    }
}
