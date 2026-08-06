use crate::routes::RoutesMenu;

use dioxus::prelude::*;
use dsot_shared_ui::assets::LOGO_IMG;

static CSS: Asset = asset!("/assets/styles/widgets/left_panel.css");

#[component]
pub fn DesktopLeftPanel() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

        aside {
            "data-component": "desktop_left_panel",
            img {
                src: LOGO_IMG,
            }
            RoutesMenu {  }
        }
    }
}
