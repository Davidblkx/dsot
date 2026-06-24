mod button;

use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdPanelLeft, LdPanelRight},
};

use crate::layout::LayoutState;
use button::ButtonFooter;

static CSS: Asset = asset!("/assets/styles/widgets/footer.css");

#[component]
pub fn DesktopFooter() -> Element {
    let mut layout = use_context::<LayoutState>();

    rsx! {
        document::Link { rel: "stylesheet", href: CSS }
        footer {
            "data-component": "desktop_footer",
            span {
                class: "left container",
                ButtonFooter {
                    active: layout.left_panel.clone(),
                    click: move || {
                        layout.left_panel.toggle();
                    },
                    icon: rsx! { Icon { icon: LdPanelLeft } }
                }
            }

            span {
                class: "center container"
            }

            span {
                class: "right container",
                ButtonFooter {
                    click: move || {
                        layout.right_panel.toggle();
                    },
                    icon: rsx! { Icon { icon: LdPanelRight } }
                }
            }
        }
    }
}
