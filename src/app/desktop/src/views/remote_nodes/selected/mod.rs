use dioxus::prelude::*;

#[component]
pub fn RemoteNodeSelected() -> Element {
    rsx! {
        div {
            class: "selected_node"
        }
    }
}
