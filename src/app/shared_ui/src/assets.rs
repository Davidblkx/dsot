///!
/// Imports static assets and exposes them
use dioxus::prelude::*;

#[used]
pub static FONT_SATOSHI: Asset = asset!(
    "/assets/fonts/Satoshi-Light.woff2",
    AssetOptions::builder().with_hash_suffix(false)
);

#[used]
pub static FONT_TANKER: Asset = asset!(
    "/assets/fonts/Tanker-Regular.woff2",
    AssetOptions::builder().with_hash_suffix(false)
);

pub const FAVICON: Asset = asset!("/assets/favicon.ico");

pub const ROOT_CSS: Asset = asset!("/assets/styles/root.css");
pub const PRIMITIVES_CSS: Asset = asset!("/assets/styles/primitives.css");

pub const LOGO_IMG: Asset = asset!("/assets/imgs/logo.png");

#[component]
pub fn DsotDefaultLinks(styles: &'static [Asset]) -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: ROOT_CSS }
        document::Link { rel: "stylesheet", href: PRIMITIVES_CSS }

        for asset in styles {
            document::Link { rel: "stylesheet", href: *asset }
        }
    }
}
