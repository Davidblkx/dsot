mod layout;
mod routes;
mod views;
mod widgets;

use dioxus::{mobile::Config, prelude::*};

use dsot_lib::Capability;
use dsot_shared_ui::assets::DsotDefaultLinks;

const UI_STYLES: &[Asset] = &[
    asset!("/assets/styles/layout.css"),
    asset!("/assets/styles/view/view.css"),
];

#[tokio::main]
async fn main() {
    let state = dsot_lib::DsotCoreInitOptions::new()
        .with_cap(Capability::new().with_disk_access().with_network_access())
        .with_debug(true)
        .initialize()
        .await
        .unwrap_or_else(|e| panic!("Failed to initialize state: {}", e));

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
