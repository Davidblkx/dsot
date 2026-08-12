use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdFile, LdLink, LdMusic, LdNotebook, LdUser};
use dsot_lib::{
    dsot_model::{InboxStatus, InboxValue},
    state::inbox::InboxItemValue,
};
use dsot_shared_ui::widgets::icon::*;

#[component]
pub fn InboxItem(item: InboxItemValue) -> Element {
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

    let (action1, action2) = match status {
        InboxStatus::Pending => (
            rsx! {
                button {
                    class: "cancel danger",
                    "Cancel"
                }
            },
            rsx! {
                button {
                    class: "done success",
                    "Done"
                }
            },
        ),
        _ => (
            rsx! {},
            rsx! {
                button {
                    class: "reopen info",
                    "Reopen"
                }
            },
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
