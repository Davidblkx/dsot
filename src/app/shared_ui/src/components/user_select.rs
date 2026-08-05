pub use dioxus::prelude::*;

use dsot_lib::{DsotCore, repository::UserRepository};

#[component]
pub fn UserSelect(is_empty: Element, on_user_select: EventHandler<String>) -> Element {
    let core = use_context::<DsotCore>();
    let users = use_resource(move || {
        let repo = core.repo.clone();

        async move {
            match repo.list_users().await {
                Ok(users) => {
                    if !users.is_empty() {
                        Some(users)
                    } else {
                        None
                    }
                }
                Err(err) => {
                    ::log::error!("Failed to list users: {}", err);
                    None
                }
            }
        }
    });

    rsx! {
        div {
            "data-component": "user-select",
            match &*users.read() {
                Some(Some(users)) => rsx! {
                    div {
                        h2 { "Select User" }
                        div {
                            class: "list",
                            ul {
                                for u in users.iter() {
                                    li {
                                        key: "{u}",
                                        onclick: {
                                            let id = u.clone();
                                            move |_| {
                                                on_user_select.call(id.clone());
                                            }
                                        },
                                        "{u}"
                                    }
                                }
                            }
                        }
                    }
                },
                Some(None) => is_empty,
                None => rsx! { div { class: "loading", "Loading users..." } }
            }
        }
    }
}
