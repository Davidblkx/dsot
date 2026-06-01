use dioxus::{
    desktop::{Config, WindowBuilder, muda::Menu},
    prelude::*,
};

mod layout;
mod routes;
mod views;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    let menu = Menu::new();

    let cfg = Config::default()
        .with_window(WindowBuilder::new().with_title("DSOT"))
        .with_menu(menu);

    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<routes::Routes> {}
    }
}
