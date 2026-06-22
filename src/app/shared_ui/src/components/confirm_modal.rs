use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdInfo, LdOctagonX, LdTriangleAlert},
};

use super::modal::*;

#[derive(Debug, Props, PartialEq, Clone)]
pub struct ConfirmModalProps {
    pub title: Signal<String>,
    #[props(default)]
    pub is_open: Signal<bool>,
    pub content: ConfirmModalContentType,
    #[props(default)]
    pub buttons: ConfirmModalButtonType,
    pub on_ok: Option<EventHandler<()>>,
    pub on_cancel: Option<EventHandler<()>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ConfirmModalContentType {
    Info(String),
    Warn(String),
    Error(String),
    Custom(Element),
}

#[derive(Debug, PartialEq, Clone, Default)]
pub enum ConfirmModalButtonType {
    Ok,
    OkCancel,
    #[default]
    CancelOk,
    Custom(Element),
}

#[component]
pub fn ConfirmModal(props: ConfirmModalProps) -> Element {
    let content = match props.content {
        ConfirmModalContentType::Info(text) => rsx! {
            span {
                class: "icon",
                Icon { icon: LdInfo }
            }
            span {
                class: "text",
                "{text}"
            }
        },
        ConfirmModalContentType::Warn(text) => rsx! {
            span {
                class: "icon",
                Icon { icon: LdTriangleAlert }
            }
            span {
                class: "text",
                "{text}"
            }
        },
        ConfirmModalContentType::Error(text) => rsx! {
            span {
                class: "icon",
                Icon { icon: LdOctagonX }
            }
            span {
                class: "text",
                "{text}"
            }
        },
        ConfirmModalContentType::Custom(content) => content,
    };

    let buttons = match props.buttons {
        _ => rsx! {},
    };

    rsx! {
        Modal {
            show_button: false,
            close_on_click_outside: false,
            is_open: props.is_open,
            div {
                "data-component": "confirm_modal",
                header {
                    "{props.title}"
                },
                div {
                    class: "body",
                    {content}
                },
                div {
                    class: "footer",
                    {buttons}
                }
            }
        }
    }
}
