use dioxus::prelude::*;

use super::model::{AddInboxValueState, AddInboxValueStateStoreExt};

#[component]
pub fn FileInput(mut state: Store<AddInboxValueState>) -> Element {
    rsx! {
        div {
            class: "form-group",
            label { "File or Directory" }
            div {
                class: "file-input-wrapper",
                div {
                    class: "file-input-buttons",
                    button {
                        class: "btn btn-secondary",
                        onclick: move |_| {
                            if let Some(path) = rfd::FileDialog::new().pick_file() {
                                state.text().set(path.display().to_string());
                            }
                        },
                        "Select File"
                    }
                    button {
                        class: "btn btn-secondary",
                        onclick: move |_| {
                            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                state.text().set(path.display().to_string());
                            }
                        },
                        "Select Directory"
                    }
                }
                span {
                    class: "selected_files",
                    "{state.text()}"
                }
            }
        }
    }
}

#[component]
pub fn ArtistInput(mut state: Store<AddInboxValueState>) -> Element {
    rsx! {
        div {
            class: "form-group",
            label { "Artist Name" }
            input {
                type: "text",
                placeholder: "Enter artist name...",
                value: "{state.text()}",
                oninput: move |evt| state.text().set(evt.value())
            }
        }
    }
}

#[component]
pub fn AlbumInput(mut state: Store<AddInboxValueState>) -> Element {
    let year_val = match state.album_year()() {
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
                value: "{state.text()}",
                oninput: move |evt| state.text().set(evt.value())
            }
        }
        div {
            class: "form-group",
            label { "Artist Name" }
            input {
                type: "text",
                placeholder: "Enter artist name...",
                value: "{state.album_artist()}",
                oninput: move |evt| state.album_artist().set(evt.value())
            }
        }
        div {
            class: "form-group",
            label { "Year" }
            input {
                type: "number",
                placeholder: "Enter year...",
                value: "{year_val}",
                oninput: move |evt| {
                    if let Ok(year) = evt.value().parse::<i64>() {
                        state.album_year().set(Some(year));
                    } else {
                        state.album_year().set(None);
                    }
                }
            }
        }
    }
}

#[component]
pub fn LinkInput(mut state: Store<AddInboxValueState>) -> Element {
    rsx! {
        div {
            class: "form-group",
            label { "URL Link" }
            input {
                type: "text",
                placeholder: "https://example.com",
                value: "{state.text()}",
                oninput: move |evt| {
                    let mut val = evt.value();
                    let old_val = state.text()();
                    // Detect paste (length increased by more than 1)
                    if val.len() > old_val.len() + 1 && !val.contains("://") {
                        val = format!("https://{}", val);
                    }
                    state.text().set(val);
                }
            }
        }
    }
}

#[component]
pub fn OtherInput(mut state: Store<AddInboxValueState>) -> Element {
    rsx! {
        div {
            class: "form-group",
            label { "Details" }
            textarea {
                value: "{state.text()}",
                placeholder: "Enter details here...",
                oninput: move |evt| state.text().set(evt.value())
            }
        }
    }
}
