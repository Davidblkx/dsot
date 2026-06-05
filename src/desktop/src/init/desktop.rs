use dioxus::{
    desktop::{Config, WindowBuilder, muda::Menu},
    prelude::*,
};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

pub fn init_desktop() {
    let state = match dsot_lib::DsotState::init(dsot_lib::DsotStateInitOptions {
        debug: true,
        config_file: None,
    }) {
        Ok(s) => s,
        Err(e) => panic!("Failed to initialize state: {}", e),
    };

    let menu = Menu::new();

    let cfg = Config::default()
        .with_window(WindowBuilder::new().with_title("DSOT"))
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
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<crate::routes::Routes> {}
    }
}
