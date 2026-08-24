mod helpers;
mod layout;
mod routes;
mod views;
mod widgets;

use dioxus::{
    desktop::{Config, WindowBuilder, muda::Menu},
    prelude::*,
};
use dsot_lib::Capability;
use dsot_shared_ui::{assets::DsotDefaultLinks, components::PortalHost};

use crate::layout::LayoutState;

const UI_STYLES: &[Asset] = &[
    asset!("/assets/styles/layout.css"),
    asset!("/assets/styles/view/view.css"),
];

#[tokio::main]
async fn main() {
    let state = dsot_lib::DsotCoreInitOptions::new()
        .with_cap(
            Capability::new()
                .with_disk_access()
                .with_network_access()
                .with_full_disk_access(),
        )
        .with_debug(true)
        .initialize()
        .await
        .unwrap_or_else(|e| panic!("Failed to initialize state: {}", e));

    let menu = Menu::new();

    let cfg = Config::default()
        .with_window(
            WindowBuilder::new()
                .with_title("DSOT")
                .with_decorations(false),
        )
        .with_menu(menu);

    LaunchBuilder::desktop()
        .with_context(state)
        .with_cfg(cfg)
        .launch(App);
}

#[component]
fn App() -> Element {
    helpers::max_state::track_state();
    dsot_shared_ui::components::use_portals();

    let state = use_context_provider::<LayoutState>(|| LayoutState::default());

    let left_panel = use_memo(move || {
        if *state.left_panel.read() {
            "true"
        } else {
            "false"
        }
    });
    set_attribute!("data-layout-left-panel", left_panel);

    let right_panel = use_memo(move || {
        if *state.right_panel.read() {
            "true"
        } else {
            "false"
        }
    });
    set_attribute!("data-layout-right-panel", right_panel);

    rsx! {
        DsotDefaultLinks { styles: UI_STYLES }

        PortalHost {  }

        Router::<routes::Routes> {}
    }
}
