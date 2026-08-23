use dioxus::prelude::*;
use dsot_lib::{DsotCore, repository::InboxRepository};

use super::model::{InboxFormItem, InboxFormState};

pub type InboxFormStore = Store<InboxFormState>;

pub fn use_inbox_form_state(item: InboxFormItem) -> InboxFormStore {
    let core = use_context::<DsotCore>();
    let store = use_store(|| InboxFormState::new(item.clone()));

    if let InboxFormItem::Edit(id) = item {
        use_future(move || {
            let repo = core.repo.clone();
            let mut store = store.clone();

            async move {
                let item = match repo.get_inbox_item(id).await {
                    Ok(item) => item,
                    Err(err) => {
                        ::log::error!("Failed to get inbox item: {}", err);
                        store
                            .write()
                            .add_error("Error fetching item from repository");
                        return;
                    }
                };
                store.write().from_inbox_item(item.value);
            }
        });
    }

    store
}
