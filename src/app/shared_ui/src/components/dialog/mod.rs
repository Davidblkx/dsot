use dioxus::prelude::*;

use super::modal::*;

mod buttons;
mod content;

pub use buttons::*;
pub use content::*;

#[derive(Debug, Props, PartialEq, Clone)]
pub struct DialogProps {
    pub title: Signal<String>,
    #[props(default)]
    pub is_open: Signal<bool>,
    pub content: DialogContentType,
    #[props(default)]
    pub buttons: DialogButtonType,
    pub on_ok: Option<EventHandler<()>>,
    pub on_cancel: Option<EventHandler<()>>,
}

static CSS: Asset = asset!("/assets/styles/components/dialog.css");

#[component]
pub fn Dialog(props: DialogProps) -> Element {
    let content = props.content.get_content();
    let content_type = props.content.get_content_type();

    let buttons = props.buttons.get_buttons_element();

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: CSS,
        }

        Modal {
            show_button: false,
            close_on_click_outside: false,
            is_open: props.is_open,
            div {
                "data-component": "dialog",
                "data-content-type": "{content_type}",
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
