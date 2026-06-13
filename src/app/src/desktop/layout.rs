use dioxus::prelude::*;

use super::routes::Routes;
use super::widgets::{
    DesktopFooter, DesktopLeftPanel, DesktopRightPanel, DesktopTopBar, ResizeBox,
};

#[component]
pub fn Layout() -> Element {
    rsx! {
        DesktopTopBar {  }

        DesktopLeftPanel {  }

        main {
            "data-component": "desktop_main",
            Link {
                to: Routes::HomeView,
                "Home"
            }
            Link {
                to: Routes::ConfigView,
                "Config"
            }
            Link {
                to: Routes::InboxView,
                "Inbox"
            }
            Outlet::<Routes> {}
        }

        DesktopRightPanel {  }

        DesktopFooter {  }

        ResizeBox {  }
    }
}
