use dioxus::prelude::*;
use super::model::{AddInboxValueState, AddInboxValueStateStoreExt};
pub fn test(mut state: Store<AddInboxValueState>) -> Element {
    rsx! {
        textarea {
            value: "{state.text()}",
            onchange: move |evt| {
                state.text().set(evt.value());
            }
        }
    }
}
