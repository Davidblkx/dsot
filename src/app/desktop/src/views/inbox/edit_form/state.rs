use dioxus::prelude::*;

use super::model::{InboxFormItem, InboxFormState};

pub fn use_inbox_form_state(item: InboxFormItem) -> Store<InboxFormState> {
    use_store(|| InboxFormState::new(item))
}
