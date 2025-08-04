//! Desktop client application for the DSOT project.
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // Global app resources
        "Hello, Desktop!"
    }
}
