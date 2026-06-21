use crate::routes::Routes;
use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdHome, LdInbox, LdRouter},
};
use dsot_lib::DsotState;
use dsot_shared_ui::{
    assets::LOGO_IMG,
    components::{Menu, MenuItem},
};

static CSS: Asset = asset!("/assets/styles/widgets/left_panel.css");

#[component]
pub fn DesktopLeftPanel() -> Element {
    let dsot = use_context::<DsotState>();
    let nav = navigator();
    let current_route = use_route::<Routes>();

    let goto_home = move || nav.push(Routes::HomeView);
    let goto_inbox = move || nav.push(Routes::InboxView);
    let goto_remote = move || nav.push(Routes::RemoteView);

    let is_route = move |r: Routes| if r == current_route { true } else { false };
    let show_remote = dsot.network.is_some();

    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

        aside {
            "data-component": "desktop_left_panel",
            img {
                src: LOGO_IMG,
            }
            Menu {
                MenuItem {
                    title: "Home",
                    active: is_route(Routes::HomeView),
                    click: move |_| {
                        goto_home();
                    },
                    icon: rsx! {
                        Icon {
                            icon: LdHome
                        }
                    }
                }
                MenuItem {
                    title: "Inbox",
                    active: is_route(Routes::InboxView),
                    click: move |_| {
                        goto_inbox();
                    },
                    icon: rsx! {
                        Icon {
                            icon: LdInbox
                        }
                    }
                }

                if show_remote {
                    MenuItem {
                        title: "Remote",
                        active: is_route(Routes::RemoteView),
                        click: move |_| {
                            goto_remote();
                        },
                        icon: rsx! {
                            Icon {
                                icon: LdRouter
                            }
                        }
                    }
                }
            }
        }
    }
}
