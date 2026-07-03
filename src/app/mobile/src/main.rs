mod layout;
mod routes;

use dioxus::{mobile::Config, prelude::*};

use dsot_shared_ui::assets::DsotDefaultLinks;

const UI_STYLES: &[Asset] = &[asset!("/assets/styles/layout.css")];

#[tokio::main]
async fn main() {
    let state = match dsot_lib::DsotState::init(dsot_lib::DsotStateInitOptions {
        debug: true,
        config_file: None,
        is_mobile: true,
    })
    .await
    {
        Ok(s) => s,
        Err(e) => panic!("Failed to initialize state: {}", e),
    };

    let config = Config::default();

    LaunchBuilder::mobile()
        .with_context(state)
        .with_cfg(config)
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        DsotDefaultLinks { styles: UI_STYLES }
        Router::<routes::Routes> {}
    }
}
