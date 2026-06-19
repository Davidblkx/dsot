use crate::views::inbox::InboxStore;
use dioxus::prelude::*;

#[derive(Debug, Clone)]
pub struct ViewContext {
    pub inbox: Store<InboxStore>,
}

pub fn provide_view_context() {
    use_context_provider(|| ViewContext {
        inbox: use_store(|| InboxStore::new()),
    });
}
