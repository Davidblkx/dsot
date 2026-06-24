use dioxus::prelude::*;

#[component]
pub fn Favourites() -> Element {
    rsx! {
        div {
            "data-component": "favourites",
        }
    }
}
