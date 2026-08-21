use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct HeaderProps {
    pub title: String,
    pub icon: Element,
    pub children: Element,
}

#[component]
pub fn Header(props: HeaderProps) -> Element {
    rsx! {
        header {
            span {
                class: "icon",
                {props.icon}
            }
            h1 {
                "{props.title}"
            }
            {props.children}
        }
    }
}
