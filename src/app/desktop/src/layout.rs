use dioxus::prelude::*;

use dsot_shared_ui::views::UserView;

use super::routes::Routes;
use super::widgets::layout::{
    DesktopFooter, DesktopFrame, DesktopLeftPanel, DesktopRightPanel, DesktopTopBar,
};

#[component]
pub fn Layout() -> Element {
    rsx! {
        DesktopFrame {  }

        DesktopTopBar {  }

        UserView {
            DesktopLeftPanel {  }

            main {
                "data-component": "desktop_main",
                Outlet::<Routes> {}
            }

            DesktopRightPanel {  }

            DesktopFooter {  }
        }
    }
}

#[derive(Debug, Clone)]
pub struct LayoutState {
    pub left_panel: Signal<bool>,
    pub right_panel: Signal<bool>,
}

impl Default for LayoutState {
    fn default() -> Self {
        Self {
            left_panel: Signal::new(true),
            right_panel: Signal::new(false),
        }
    }
}
