mod helpers;
mod layout;
mod routes;
mod state;
mod views;
mod widgets;

use dioxus::{
    desktop::{Config, WindowBuilder, muda::Menu},
    prelude::*,
};
use dsot_shared_ui::components::PortalHost;
use state::AppStateProvier;

use crate::layout::LayoutState;

const UI_STYLES: &[Asset] = &[
    dsot_shared_ui::assets::ROOT_CSS,
    dsot_shared_ui::assets::PRIMITIVES_CSS,
    asset!("/assets/styles/layout.css"),
    asset!("/assets/styles/view/view.css"),
];

#[tokio::main]
async fn main() {
    let state = match dsot_lib::DsotState::init(dsot_lib::DsotStateInitOptions {
        debug: true,
        config_file: None,
        is_mobile: false,
    })
    .await
    {
        Ok(s) => s,
        Err(e) => panic!("Failed to initialize state: {}", e),
    };

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
        document::Link { rel: "icon", href: dsot_shared_ui::assets::FAVICON }
        for style in UI_STYLES {
            document::Link { rel: "stylesheet", href: *style }
        }

        PortalHost {  }

        AppStateProvier {
            Router::<routes::Routes> {}
        }

    }
}
