use dioxus::prelude::*;
use dioxus_free_icons::{Icon, IconShape};

/// Helper to render an icon using the [`Icon`] component.
pub fn icon<T: IconShape + Clone + PartialEq + 'static>(i: T) -> Element {
    rsx! {
        Icon {
            class: "dsot_icon",
            icon: i
        }
    }
}
