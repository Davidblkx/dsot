use dioxus::prelude::*;
use dsot_lib::DsotState;

#[component]
pub fn ConfigView() -> Element {
    let state = use_context::<DsotState>();
    let name = state.config.value.user.clone();

    rsx! {
        h1 { "Config" }
        h3 { "{name}" }
    }
}
