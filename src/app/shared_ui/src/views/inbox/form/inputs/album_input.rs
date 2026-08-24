use dioxus::prelude::*;

use crate::views::inbox::form::{model::InboxFormStateStoreExt, state::InboxFormStore};

#[component]
pub fn AlbumInput() -> Element {
    let store = use_context::<InboxFormStore>();

    let year_val = match store.album_year()() {
        Some(y) => y.to_string(),
        None => String::new(),
    };
    rsx! {
        div {
            class: "form-group",
            label { "Album Name" }
            input {
                type: "text",
                placeholder: "Enter album name...",
                value: "{store.text()}",
                onchange: move |evt| store.text().set(evt.value())
            }
        }
        div {
            class: "form-group",
            label { "Artist Name" }
            input {
                type: "text",
                placeholder: "Enter artist name...",
                value: "{store.album_artist()}",
                onchange: move |evt| store.album_artist().set(evt.value())
            }
        }
        div {
            class: "form-group",
            label { "Year" }
            input {
                type: "number",
                placeholder: "Enter year...",
                value: "{year_val}",
                onchange: move |evt| {
                    if let Ok(year) = evt.value().parse::<u32>() {
                        store.album_year().set(Some(year));
                    } else {
                        store.album_year().set(None);
                    }
                }
            }
        }
    }
}
