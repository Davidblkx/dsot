pub mod model;
pub mod state;
pub mod type_selector;

use dioxus::prelude::*;

use model::InboxFormItem;
use type_selector::InboxFormTypeSelector;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct InboxFormProps {
    pub item: InboxFormItem,
}

#[component]
pub fn InboxForm(props: InboxFormProps) -> Element {
    let title = match props.item {
        InboxFormItem::New => "New Inbox Item",
        InboxFormItem::Edit(_) => "Edit Inbox Item",
    };
    let state = state::use_inbox_form_state(props.item);

    rsx! {
        form {
            "data-component": "inbox_form",
            h2 {
                class: "title",
                "{title}"
            }
            InboxFormTypeSelector { state }
        }
    }
}
