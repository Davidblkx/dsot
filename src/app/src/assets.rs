///!
/// Imports static assets not referenced directly
use dioxus::prelude::*;

#[used]
static FONT_SATOSHI: Asset = asset!(
    "/assets/fonts/Satoshi-Light.woff2",
    AssetOptions::builder().with_hash_suffix(false)
);

#[used]
static FONT_TANKER: Asset = asset!(
    "/assets/fonts/Tanker-Regular.woff2",
    AssetOptions::builder().with_hash_suffix(false)
);
