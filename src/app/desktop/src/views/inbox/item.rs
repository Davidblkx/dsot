use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dsot_lib::dsot_model::InboxValue;

use crate::state::inbox::InboxViewItem;

#[component]
pub fn InboxItem(item: InboxViewItem) -> Element {
    let InboxViewItem { id, value, status } = item;

    let (icon, text) = match value {
        InboxValue::File(name) => (
            rsx! { Icon { icon: dioxus_free_icons::icons::ld_icons::LdFile } },
            rsx! { "{name}" },
        ),
        InboxValue::Artist(name) => (
            rsx! { Icon { icon: dioxus_free_icons::icons::ld_icons::LdUser } },
            rsx! { "{name}" },
        ),
        InboxValue::Link(name) => (
            rsx! { Icon { icon: dioxus_free_icons::icons::ld_icons::LdLink } },
            rsx! { "{name}" },
        ),
        InboxValue::Other(name) => (
            rsx! { Icon { icon: dioxus_free_icons::icons::ld_icons::LdAsterisk } },
            rsx! { "{name}" },
        ),
        InboxValue::Album {
            album,
            artist,
            year,
        } => {
            let artist = if artist.is_empty() {
                "".to_string()
            } else {
                format!(" by {}", artist)
            };

            let year = match year {
                Some(year) => format!(" ({})", year),
                None => "".to_string(),
            };

            (
                rsx! { Icon { icon: dioxus_free_icons::icons::ld_icons::LdDisc3 } },
                rsx! { "{album}{artist}{year}" },
            )
        }
    };
    let status = status.as_db_str().to_string();

    rsx! {
        div {
            "data-component": "inbox_view_item",
            "data-status": "{status}",
            "data-id": "{id}",
            span {
                class: "icon",
                {icon}
            }
            span {
                class: "text",
                {text}
            }
        }
    }
}
