use dioxus::prelude::*;
use super::model::AddInboxValueState;
pub fn test_fn(state: Store<AddInboxValueState>) -> Element {
    rsx! {
        input {
            value: "{state.text()}"
        }
    }
}
