use dioxus::{mobile::Config, prelude::*};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

pub async fn init_mobile() {
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
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<crate::routes::Routes> {}
    }
}
