use dioxus::prelude::*;

#[component]
pub fn DesktopFooter() -> Element {
    rsx! {
        footer {
            "data-component": "desktop_footer"
        }
    }
}
