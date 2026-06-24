use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdInfo, LdOctagonX, LdTriangleAlert},
};

#[derive(Debug, PartialEq, Clone)]
pub enum DialogContentType {
    Info(String),
    Warn(String),
    Error(String),
    Custom(Element),
}

impl DialogContentType {
    pub fn get_content(&self) -> Element {
        match self {
            DialogContentType::Info(text) => rsx! {
                span {
                    class: "icon",
                    Icon { icon: LdInfo }
                }
                span {
                    class: "text",
                    "{text}"
                }
            },
            DialogContentType::Warn(text) => rsx! {
                span {
                    class: "icon",
                    Icon { icon: LdTriangleAlert }
                }
                span {
                    class: "text",
                    "{text}"
                }
            },
            DialogContentType::Error(text) => rsx! {
                span {
                    class: "icon",
                    Icon { icon: LdOctagonX }
                }
                span {
                    class: "text",
                    "{text}"
                }
            },
            DialogContentType::Custom(content) => content.to_owned(),
        }
    }

    pub fn get_content_type(&self) -> &'static str {
        match self {
            DialogContentType::Info(_) => "info",
            DialogContentType::Warn(_) => "warn",
            DialogContentType::Error(_) => "error",
            DialogContentType::Custom(_) => "custom",
        }
    }

    pub fn info<T: ToString>(message: T) -> Self {
        DialogContentType::Info(message.to_string())
    }

    pub fn warn<T: ToString>(message: T) -> Self {
        DialogContentType::Warn(message.to_string())
    }

    pub fn error<T: ToString>(message: T) -> Self {
        DialogContentType::Error(message.to_string())
    }

    pub fn custom(content: Element) -> Self {
        DialogContentType::Custom(content)
    }
}
