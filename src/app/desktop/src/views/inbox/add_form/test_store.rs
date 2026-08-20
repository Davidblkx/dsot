use dioxus::prelude::*;
use super::model::AddInboxValueState;
pub fn test_fn(state: Store<AddInboxValueState>) {
    let mut text_sig = state.text();
    text_sig.set("hello".to_string());
}
