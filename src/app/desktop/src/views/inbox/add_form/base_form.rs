use dioxus::prelude::*;

use super::{form_inputs::FormInputs, model::AddInboxValueState, type_selector::TypeSelector};

#[component]
pub fn InboxValueForm(state: Store<AddInboxValueState>) -> Element {
    rsx! {
        div {
            class: "view_inbox__add_inbox_value_form",
            TypeSelector {
                state: state
            }
            FormInputs {
                state: state
            }
        }
    }
}
