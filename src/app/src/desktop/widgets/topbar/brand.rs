use dioxus::prelude::*;

static LOGO: Asset = asset!("/assets/imgs/icon_256.png");

#[component]
pub fn Brand() -> Element {
    rsx! {
        span {
            "data-component": "brand",
            img {
                src: LOGO
            }
            span {
                "DSOT"
            }
        }
    }
}
