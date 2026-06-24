use dioxus::prelude::*;

#[derive(Debug, PartialEq, Clone, Default)]
pub enum DialogButtonType {
    Ok,
    #[default]
    CancelOk,
    Custom(Element),
}

impl DialogButtonType {
    pub fn get_buttons_element(&self) -> Element {
        rsx! {}
    }
}
