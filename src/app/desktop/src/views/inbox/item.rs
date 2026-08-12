use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdFile, LdLink, LdMusic, LdNotebook, LdUser};
use dsot_lib::{dsot_model::InboxValue, state::inbox::InboxItemValue};
use dsot_shared_ui::widgets::icon::*;

#[component]
pub fn InboxItem(item: InboxItemValue) -> Element {
    let (icon, content) = match item.value {
        InboxValue::File(path) => (icon(LdFile), rsx! { "{path}" }),
        InboxValue::Artist(artist) => (icon(LdUser), rsx! { "{artist}" }),
        InboxValue::Link(url) => (icon(LdLink), rsx! { "{url}" }),
        InboxValue::Album {
            album,
            artist,
            year,
        } => {
            let icon = icon(LdMusic);
            let year = year.map(|y| format!(" ({y})")).unwrap_or_default();
            (icon, rsx! { "{album} by {artist}{year}" })
        }
        InboxValue::Other(note) => (icon(LdNotebook), rsx! { "{note}" }),
    };

    rsx! {
        div {
            class: "icon",
            {icon}
        }
        div {
            class: "content",
            {content}
        }
    }
}
