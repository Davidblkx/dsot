mod form_inputs;
mod inputs;
pub mod model;
mod save_button;
pub mod state;
mod type_selector;

use dioxus::prelude::*;

use form_inputs::FormInputs;
use model::InboxFormItem;
use save_button::SaveButton;
use type_selector::InboxFormTypeSelector;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct InboxFormProps {
    #[props(default)]
    pub item: InboxFormItem,
    #[props(default)]
    pub platform_file_input: Option<Element>,
    #[props(default)]
    pub on_save: EventHandler<()>,
}

#[component]
pub fn InboxForm(props: InboxFormProps) -> Element {
    let title = match props.item {
        InboxFormItem::New => "New Inbox Item",
        InboxFormItem::Edit(_) => "Edit Inbox Item",
    };
    let state = state::use_inbox_form_state(props.item);
    provide_context(state);

    rsx! {
        form {
            "data-component": "inbox_form",
            h2 {
                class: "title",
                "{title}"
            }
            InboxFormTypeSelector { }
            FormInputs {
                platform_file_input: props.platform_file_input
            }
            SaveButton {
                on_save: props.on_save
            }
        }
    }
}
