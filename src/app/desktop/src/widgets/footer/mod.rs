mod button;

use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdPanelLeft, LdPanelRight},
};

use crate::layout::LayoutState;
use button::ButtonFooter;

#[component]
pub fn DesktopFooter() -> Element {
    let mut layout = use_context::<LayoutState>();

    rsx! {
        footer {
            "data-component": "desktop_footer",
            span {
                class: "left",
                ButtonFooter {
                    on_click: move || {
                        layout.left_panel.toggle();
                    },
                    icon: rsx! { Icon { icon: LdPanelLeft } }
                }
            }
            span {
                class: "right",
                ButtonFooter {
                    on_click: move || {
                        layout.right_panel.toggle();
                    },
                    icon: rsx! { Icon { icon: LdPanelRight } }
                }
            }
        }
    }
}
