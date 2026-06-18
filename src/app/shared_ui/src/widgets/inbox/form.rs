use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdAsterisk, LdFile, LdLink, LdMusic, LdUser},
};
use dsot_lib::dsot_model::{InboxItem, InboxValue};

static FORM_CSS: Asset = asset!("/assets/styles/widgets/inbox.css");

#[derive(Debug, Clone, PartialEq, Props)]
pub struct FormAddInboxItemProps {
    on_save: Option<EventHandler<InboxItem>>,
    #[props(default = true)]
    clear_on_save: bool,
}

#[derive(Debug, Clone, PartialEq)]
enum InnerInboxItemType {
    Artist,
    Album,
    File,
    Link,
    Other,
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SelectInboxTypeProps {
    selected: InnerInboxItemType,
    on_select: EventHandler<InnerInboxItemType>,
}

#[component]
fn SelectInboxType(props: SelectInboxTypeProps) -> Element {
    rsx! {
        ul {
            "data-component": "select_inbox_type",
            li {
                "data-selected": if InnerInboxItemType::Artist == props.selected { "true" } else { "false" },
                onclick: move |_| props.on_select.call(InnerInboxItemType::Artist),
                span {
                    class: "icon",
                    Icon {
                        icon: LdUser,
                    }
                }
                span {
                    class: "text",
                    "Artist"
                }
            }
            li {
                "data-selected": if InnerInboxItemType::Album == props.selected { "true" } else { "false" },
                onclick: move |_| props.on_select.call(InnerInboxItemType::Album),
                span {
                    class: "icon",
                    Icon {
                        icon: LdMusic,
                    }
                }
                span {
                    class: "text",
                    "Album"
                }
            }
            li {
                "data-selected": if InnerInboxItemType::File == props.selected { "true" } else { "false" },
                onclick: move |_| props.on_select.call(InnerInboxItemType::File),
                span {
                    class: "icon",
                    Icon {
                        icon: LdFile,
                    }
                }
                span {
                    class: "text",
                    "File"
                }
            }
            li {
                "data-selected": if InnerInboxItemType::Link == props.selected { "true" } else { "false" },
                onclick: move |_| props.on_select.call(InnerInboxItemType::Link),
                span {
                    class: "icon",
                    Icon {
                        icon: LdLink,
                    }
                }
                span {
                    class: "text",
                    "Link"
                }
            }
            li {
                "data-selected": if InnerInboxItemType::Other == props.selected { "true" } else { "false" },
                onclick: move |_| props.on_select.call(InnerInboxItemType::Other),
                span {
                    class: "icon",
                    Icon {
                        icon: LdAsterisk,
                    }
                }
                span {
                    class: "text",
                    "Other"
                }
            }
        }
    }
}

#[component]
pub fn FormAddInboxItem(props: FormAddInboxItemProps) -> Element {
    let mut item_type = use_signal(|| InnerInboxItemType::Other);
    let mut value = use_signal(|| String::from(""));
    let mut album_artist = use_signal(|| String::from(""));
    let mut album_year_str = use_signal(|| String::from(""));
    let mut value_name = use_signal(|| "Anything".to_string());

    use_effect(move || {
        let name = match item_type() {
            InnerInboxItemType::File => "File",
            InnerInboxItemType::Artist => "Artist",
            InnerInboxItemType::Album => "Album",
            InnerInboxItemType::Link => "Link",
            InnerInboxItemType::Other => "Anything",
        };

        value_name.set(name.to_string());
    });

    let mut save = move || {
        if let Some(handler) = props.on_save.as_ref() {
            let val = value.read().clone();

            let itm_value = match *item_type.read() {
                InnerInboxItemType::File => InboxValue::File(val),
                InnerInboxItemType::Artist => InboxValue::Artist(val),
                InnerInboxItemType::Album => InboxValue::Album {
                    album: val,
                    artist: album_artist.read().clone(),
                    year: album_year_str.read().trim().parse::<u32>().ok(),
                },
                InnerInboxItemType::Link => InboxValue::Link(val),
                InnerInboxItemType::Other => InboxValue::Other(val),
            };

            match InboxItem::new(itm_value) {
                Ok(itm) => {
                    handler.call(itm);
                    if props.clear_on_save {
                        value.set("".to_string());
                        album_artist.set("".to_string());
                        album_year_str.set("".to_string());
                    }
                }
                Err(e) => log::error!("Failed to create inbox item: {}", e),
            };
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: FORM_CSS }
        form {
            "data-component": "form_add_inbox_item",
            onsubmit: |e| {
                e.prevent_default();
            },
            SelectInboxType {
                selected: item_type(),
                on_select: move |itm| {
                    item_type.set(itm);
                },
            }
            span {
                class: "form-group",
                label {
                    for: "form_inbox_value",
                    "{value_name}:"
                }
                input {
                    id: "form_inbox_value",
                    type: "text",
                    value: "{value()}",
                    oninput: move |e| {
                        value.set(e.value().clone());
                    },
                }
            }
            if item_type() == InnerInboxItemType::Album {
                span {
                    class: "form-group",
                    label {
                        for: "form_inbox_album_artist",
                        "Album Artist:"
                    }
                    input {
                        id: "form_inbox_album_artist",
                        type: "text",
                        value: "{album_artist()}",
                        oninput: move |e| {
                            album_artist.set(e.value().clone());
                        },
                    }
                }
                span {
                    class: "form-group",
                    label {
                        for: "form_inbox_album_year",
                        "Album Year:"
                    }
                    input {
                        id: "form_inbox_album_year",
                        type: "text",
                        value: "{album_year_str()}",
                        oninput: move |e| {
                            album_year_str.set(e.value().clone());
                        },
                    }
                }
            }
            button {
                type: "submit",
                onclick: move |_| {
                    save();
                },
                "Save",
            }
        }
    }
}
