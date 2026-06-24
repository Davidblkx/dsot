use dioxus::prelude::*;

#[component]
pub fn View(name: String, css: Asset, children: Element) -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: css,
        }

        div {
            "data-component": "view",
            "data-view": "{name}",

            {children}
        }
    }
}
