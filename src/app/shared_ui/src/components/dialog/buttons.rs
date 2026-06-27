use dioxus::prelude::*;

#[derive(Debug, PartialEq, Clone, Default)]
pub enum DialogButtonType {
    Ok,
    #[default]
    CancelOk,
    Custom(Element),
}

impl DialogButtonType {
    pub fn get_buttons_element(
        &self,
        on_cancel: EventHandler<()>,
        on_ok: EventHandler<()>,
    ) -> Element {
        match self {
            DialogButtonType::Ok => rsx! {
                button {
                    onclick: move |_| {
                        on_ok.call(());
                    },
                    "OK"
                }
            },
            DialogButtonType::CancelOk => rsx! {
                button {
                    onclick: move |_| {
                        on_cancel.call(());
                    },
                    "Cancel"
                }
                button {
                    onclick: move |_| {
                        on_ok.call(());
                    },
                    "OK"
                }
            },
            DialogButtonType::Custom(element) => element.clone(),
        }
    }
}
