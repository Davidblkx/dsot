use dioxus::prelude::*;
pub fn test() -> Element {
    rsx! {
        input {
            initial_value: "test",
            onchange: move |e| {}
        }
    }
}
