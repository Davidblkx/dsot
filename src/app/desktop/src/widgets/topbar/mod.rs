use dioxus::desktop::use_window;
use dioxus::prelude::*;

mod brand;
mod buttons;
mod fav;

static CSS: Asset = asset!("/assets/styles/desktop/widgets/topbar.css");

#[component]
pub fn DesktopTopBar() -> Element {
    let win = use_window();

    rsx! {
        document::Link { rel: "stylesheet", href: CSS }
        header {
            "data-component": "desktop_topbar",
            onmousedown: move |_| {
                win.drag();
            },
            brand::Brand {  }
            fav::Favourites { }
            buttons::Buttons { }
        }
    }
}
