use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdAsterisk, LdChevronDown, LdChevronUp, LdFile, LdLink, LdMusic, LdUser},
};

static FORM_CSS: Asset = asset!("/assets/styles/view/inbox.css");

#[derive(Debug, Clone, PartialEq, Default)]
enum InboxItemType {
    File,
    Artist,
    Album,
    Link,
    #[default]
    Other,
}

impl InboxItemType {
    fn label(&self) -> &'static str {
        match self {
            InboxItemType::File => "File",
            InboxItemType::Artist => "Artist",
            InboxItemType::Album => "Album",
            InboxItemType::Link => "Link",
            InboxItemType::Other => "Other",
        }
    }

    fn from_label(label: &str) -> Self {
        match label {
            "File" => InboxItemType::File,
            "Artist" => InboxItemType::Artist,
            "Album" => InboxItemType::Album,
            "Link" => InboxItemType::Link,
            _ => InboxItemType::Other,
        }
    }
}

#[component]
pub fn AddInboxItemForm() -> Element {
    let mut selected_type = use_signal(InboxItemType::default);

    // Form fields
    let mut file_path = use_signal(|| String::new());
    let mut artist_name = use_signal(|| String::new());
    
    let mut album_title = use_signal(|| String::new());
    let mut album_artist = use_signal(|| String::new());
    let mut album_year = use_signal(|| String::new());
    
    let mut link_url = use_signal(|| String::new());
    let mut other_text = use_signal(|| String::new());

    rsx! {
        document::Link { rel: "stylesheet", href: FORM_CSS }
        form {
            "data-component": "form_add_inbox_item",
            onsubmit: move |e| {
                e.prevent_default();
            },
            
            // Native Select Dropdown
            div {
                class: "form-group type-selector",
                label { for: "item_type", "Item Type" }
                select {
                    id: "item_type",
                    onchange: move |e| selected_type.set(InboxItemType::from_label(&e.value())),
                    for item in [
                        InboxItemType::File,
                        InboxItemType::Artist,
                        InboxItemType::Album,
                        InboxItemType::Link,
                        InboxItemType::Other
                    ] {
                        option {
                            value: "{item.label()}",
                            selected: "{selected_type() == item}",
                            "{item.label()}"
                        }
                    }
                }
            }

            // Dynamic Form Fields
            match selected_type() {
                InboxItemType::File => rsx! {
                    div {
                        class: "form-group",
                        label { for: "file_input", "File or Directory" }
                        div {
                            class: "file-input-wrapper",
                            input {
                                id: "file_input",
                                type: "file",
                                value: "{file_path}",
                                oninput: move |e| file_path.set(e.value()),
                            }
                        }
                    }
                },
                InboxItemType::Artist => rsx! {
                    div {
                        class: "form-group",
                        label { for: "artist_name", "Artist Name" }
                        input {
                            id: "artist_name",
                            type: "text",
                            value: "{artist_name}",
                            placeholder: "e.g., Pink Floyd",
                            oninput: move |e| artist_name.set(e.value()),
                        }
                    }
                },
                InboxItemType::Album => rsx! {
                    div { class: "form-group",
                        label { for: "album_title", "Album Title" }
                        input {
                            id: "album_title",
                            type: "text",
                            value: "{album_title}",
                            placeholder: "e.g., The Dark Side of the Moon",
                            oninput: move |e| album_title.set(e.value()),
                        }
                    }
                    div { class: "form-group",
                        label { for: "album_artist", "Album Artist" }
                        input {
                            id: "album_artist",
                            type: "text",
                            value: "{album_artist}",
                            placeholder: "e.g., Pink Floyd",
                            oninput: move |e| album_artist.set(e.value()),
                        }
                    }
                    div { class: "form-group",
                        label { for: "album_year", "Release Year" }
                        input {
                            id: "album_year",
                            type: "number",
                            value: "{album_year}",
                            placeholder: "e.g., 1973",
                            oninput: move |e| album_year.set(e.value()),
                        }
                    }
                },
                InboxItemType::Link => rsx! {
                    div { class: "form-group",
                        label { for: "link_url", "URL" }
                        input {
                            id: "link_url",
                            type: "url",
                            pattern: ".*://.*",
                            value: "{link_url}",
                            placeholder: "https://example.com/music",
                            oninput: move |e| link_url.set(e.value()),
                        }
                    }
                },
                InboxItemType::Other => rsx! {
                    div { class: "form-group",
                        label { for: "other_text", "Description" }
                        textarea {
                            id: "other_text",
                            value: "{other_text}",
                            placeholder: "Enter details here...",
                            oninput: move |e| other_text.set(e.value()),
                        }
                    }
                }
            }

            div {
                class: "submit-group",
                button {
                    type: "submit",
                    "Add to Inbox"
                }
            }
        }
    }
}
