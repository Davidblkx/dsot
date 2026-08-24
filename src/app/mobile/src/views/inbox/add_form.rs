use dioxus::prelude::*;
use dsot_shared_ui::views::inbox::form::InboxForm;

use crate::widgets::views::View;
use crate::views::inbox::COMBINED_CSS;

#[component]
pub fn AddInboxValue() -> Element {
    let navigator = use_navigator();

    rsx! {
        View {
            name: "add_inbox",
            css: COMBINED_CSS,
            div {
                class: "add-form-page",
                style: "padding: 20px; display: flex; flex-direction: column; height: 100%; gap: 20px; overflow-y: auto;",
                InboxForm {
                    on_save: move |_| {
                        navigator.push(crate::routes::Routes::InboxView);
                    }
                }
                button {
                    class: "btn-secondary",
                    style: "padding: 10px 20px; font-size: 1.1rem; margin-top: 10px;",
                    onclick: move |_| {
                        navigator.push(crate::routes::Routes::InboxView);
                    },
                    "Cancel"
                }
            }
        }
    }
}
