use dioxus::prelude::*;

#[component]
pub fn View(name: String, css: &'static [Asset], children: Element) -> Element {
    rsx! {
        for href in css {
            document::Link {
                rel: "stylesheet",
                href: href.clone(),
            }
        }

        div {
            "data-component": "view",
            "data-view": "{name}",

            {children}
        }
    }
}
