use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdFile, LdLink, LdMusic, LdNotebook, LdUser};
use dsot_lib::{
    DsotCore,
    dsot_model::{InboxStatus, InboxValue},
    state::inbox::{InboxItemValue, InboxOperations},
};
use dsot_shared_ui::widgets::icon::*;

#[component]
pub fn InboxItem(item: InboxItemValue) -> Element {
    let core = use_context::<DsotCore>();
    let InboxItemValue { id, value, status } = item;

    let (icon, content, class_name) = match value {
        InboxValue::File(path) => (icon(LdFile), rsx! { "{path}" }, "file"),
        InboxValue::Artist(artist) => (icon(LdUser), rsx! { "{artist}" }, "artist"),
        InboxValue::Link(url) => (icon(LdLink), rsx! { a { href: url, "{url}" } }, "link"),
        InboxValue::Album {
            album,
            artist,
            year,
        } => {
            let icon = icon(LdMusic);
            let year = year.map(|y| format!(" ({y})")).unwrap_or_default();
            (icon, rsx! { "{album} by {artist}{year}" }, "album")
        }
        InboxValue::Other(note) => (icon(LdNotebook), rsx! { "{note}" }, "note"),
    };

    let status_txt = status.as_db_str();

    let update_status = move |status: InboxStatus| {
        let core = core.clone();
        spawn(async move {
            match core.set_inbox_status(id, status).await {
                Ok(_) => log::debug!("Inbox status updated"),
                Err(e) => log::error!("Failed to update inbox status: {}", e),
            }
        });
    };

    let action_button = move |label: &'static str, class: &'static str, status: InboxStatus| {
        let update_status = update_status.clone();
        let status = status.clone();
        rsx! {
            button {
                class: "{class}",
                onclick: move |_| update_status(status),
                "{label}"
            }
        }
    };

    let (action1, action2) = match status {
        InboxStatus::Pending => (
            action_button("Cancel", "cancel danger", InboxStatus::Failed),
            action_button("Done", "done success", InboxStatus::Resolved),
        ),
        _ => (
            rsx! {},
            action_button("Reopen", "reopen info", InboxStatus::Pending),
        ),
    };

    rsx! {
        div {
            class: "item {class_name} status_{status_txt}",
            "data-id": "{id}",
            div {
                class: "header",
                div {
                    class: "icon",
                    {icon}
                }
                div {
                    class: "content",
                    {content}
                }
            },
            div {
                class: "actions",
                {action1}
                {action2}
            }
        }
    }
}
