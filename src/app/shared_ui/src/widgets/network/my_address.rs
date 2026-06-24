use dioxus::prelude::*;
use dsot_lib::DsotState;

#[component]
pub fn MyAddress() -> Element {
    let state = use_context::<DsotState>();
    let has_value = state.network.is_some();

    let content = match state.network {
        Some(network) => {
            let id = network.endpoint.id().to_string();

            rsx! { "{id}" }
        }
        None => {
            rsx! { "Network module is not enabled" }
        }
    };

    rsx! {
        span {
            "data-component": "my_address",
            "data-enabled": has_value,
            {content}
        }
    }
}
