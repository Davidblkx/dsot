mod layout;
mod routes;
mod widgets;

use dioxus::{
    desktop::{Config, WindowBuilder, muda::Menu},
    prelude::*,
};

const FAVICON: Asset = asset!("/assets/favicon.ico");

const UI_STYLES: &[Asset] = &[
    asset!("/assets/styles/root.css"),
    asset!("/assets/styles/desktop/layout.css"),
];

pub async fn init() {
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
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        for style in UI_STYLES {
            document::Link { rel: "stylesheet", href: *style }
        }
        Router::<routes::Routes> {}
    }
}
