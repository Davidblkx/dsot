use dioxus::prelude::*;
use super::model::{AddInboxValueState, AddInboxValueStateStoreExt};
pub fn test(mut state: Store<AddInboxValueState>) -> Element {
    let initial = state.text().peek().clone();
    rsx! {
        input {
            initial_value: "{initial}",
            oninput: move |e| state.text().set(e.value())
        }
    }
}
