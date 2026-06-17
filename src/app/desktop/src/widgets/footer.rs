use dioxus::prelude::*;

use crate::layout::LayoutState;

#[component]
pub fn DesktopFooter() -> Element {
    let mut layout = use_context::<LayoutState>();

    rsx! {
        footer {
            "data-component": "desktop_footer",
            button {
                onclick: move |_|{
                    layout.left_panel.toggle();
                },
                "Toggle"
            }
        }
    }
}
