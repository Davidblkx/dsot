use dioxus::prelude::*;
use dsot_lib::{DsotCore, state::inbox::InboxOperations};

use crate::views::inbox::form::{model::InboxFormItem, state::InboxFormStore};

async fn save_item(core: DsotCore, store: InboxFormStore) -> dsot_lib::error::Result<()> {
    log::debug!("Saving inbox item");
    let value = store.peek().into_inbox_item();
    if let InboxFormItem::Edit(id) = store.peek().item {
        core.update_inbox_item(id, value).await?;
        log::debug!("Updated inbox item: {:?}", id);
    } else {
        core.add_inbox_item(value).await?;
        log::debug!("Added new inbox item");
    }

    Ok(())
}

#[component]
pub fn SaveButton(on_save: EventHandler<()>) -> Element {
    let core = use_context::<DsotCore>();
    let store = use_context::<InboxFormStore>();

    rsx! {
        button {
            class: "btn-primary save",
            onclick: move |_| {
                let core = core.clone();
                let store = store.clone();

                async move {
                    if save_item(core, store).await.is_ok() {
                        on_save.call(());
                    }
                }
            },
            "Save"
        }
    }
}
