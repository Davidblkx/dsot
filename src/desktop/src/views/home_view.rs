use dioxus::prelude::*;

#[component]
pub fn HomeView() -> Element {
    rsx! {
        h1 { "Home" }
    }
}
