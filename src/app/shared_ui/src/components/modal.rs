use crate::components::Portal;
use dioxus::prelude::*;

static MODAL_CSS: Asset = asset!("/assets/styles/components/modal.css");

#[derive(Debug, PartialEq, Clone, Props)]
pub struct ModalProps {
    #[props(default = "Open".to_string())]
    pub text: String,
    pub button_content: Option<Element>,
    #[props(default = true)]
    pub show_button: bool,
    #[props(default = true)]
    pub close_on_click_outside: bool,
    #[props(default)]
    pub is_open: Signal<bool>,
    pub children: Element,
}

#[component]
pub fn Modal(mut props: ModalProps) -> Element {
    let value = if *props.is_open.read() {
        rsx! {
            Portal {
                div {
                    "data-component": "modal",
                    onclick: move |_| {
                        if props.close_on_click_outside {
                            props.is_open.set(false);
                        }
                    },
                    div {
                        "data-component": "modal_content",
                        onclick: move |evt| { evt.stop_propagation(); },
                        {props.children}
                    }
                }
            }
        }
    } else {
        rsx! {}
    };

    let content = if let Some(ctn) = props.button_content {
        ctn
    } else {
        rsx! { "{props.text}" }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: MODAL_CSS }

        if props.show_button {
            button {
                onclick: move |_| props.is_open.toggle(),
                {content}
            }
        }

        {value}
    }
}
