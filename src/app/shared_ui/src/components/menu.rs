use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdChevronDown, LdChevronRight};

static MENU_CSS: Asset = asset!("/assets/styles/menu.css");

#[derive(Debug, Props, Clone, PartialEq)]
pub struct MenuItemProps {
    pub title: String,
    pub click: Option<EventHandler<MouseEvent>>,
    #[props(default = false)]
    pub allow_colapse: bool,
    pub icon: Option<Element>,
    pub children: Option<Element>,
    #[props(default = false)]
    pub active: bool,
}

#[component]
pub fn MenuItem(props: MenuItemProps) -> Element {
    let mut is_open = use_signal(|| true);

    let content = if props.allow_colapse {
        let icon = if is_open() {
            rsx! {
                Icon {
                    icon: LdChevronDown,
                }
            }
        } else {
            rsx! {
                Icon {
                    icon: LdChevronRight,
                }
            }
        };

        rsx! {
            span {
                class: "header",
                onclick: move |_| {
                    is_open.set(!is_open());
                },
                span {
                    class: "icon",
                    {icon}
                }
                span {
                    class: "text",
                    "{props.title}"
                }
            }
        }
    } else {
        let icon = props.icon.unwrap_or(rsx! {});

        rsx! {
            span {
                class: "header",
                onclick: move |evt| {
                    if let Some(handler) = props.click.as_ref() {
                        handler.call(evt);
                    }
                },
                span {
                    class: "icon",
                    {icon}
                }
                span {
                    class: "text",
                    "{props.title}"
                }
            }
        }
    };

    let children = if let Some(ch) = props.children {
        rsx! {
            div {
                class: "subitems",
                {ch}
            }
        }
    } else {
        rsx! {}
    };

    rsx! {
        li {
            "data-component": "menu_item",
            "data-status": if is_open() { "open" } else { "close" },
            "data-route": if props.active { "active" } else { "" },
            {content}
            {children}
        }
    }
}

#[derive(Debug, Props, Clone, PartialEq)]
pub struct MenuProps {
    pub title: Option<String>,
    pub children: Option<Element>,
}

#[component]
pub fn Menu(props: MenuProps) -> Element {
    let children = props.children.unwrap_or(rsx! {});
    let title = match props.title {
        None => rsx! {},
        Some(text) => rsx! {
            header {
                "data-component": "menu_header",
                {text}
            }
        },
    };

    rsx! {
        document::Link { rel: "stylesheet", href: MENU_CSS }

        menu {
            "data-component": "menu",
            {title}
            {children}
        }
    }
}
