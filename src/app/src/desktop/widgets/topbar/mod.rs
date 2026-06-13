use dioxus::desktop::use_window;
use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/styles/desktop/widgets/topbar.css");

#[component]
pub fn DesktopTopBar() -> Element {
    let win = use_window();
    let win_drag = win.clone();
    let win_minimize = win.clone();
    let win_maximize = win.clone();
    let win_close = win.clone();

    rsx! {
        document::Link { rel: "stylesheet", href: CSS }
        header {
            "data-component": "desktop_topbar",
            onmousedown: move |_| {
                win_drag.drag();
            },
            h2 { "DSOT" }
            div {
                "data-component": "window_button_group",
                onmousedown: move |evt| evt.stop_propagation(),
                button {
                    "data-component": "button_minimize",
                    onclick: move |_| {
                        win_minimize.set_minimized(true);
                    },
                }
                button {
                    "data-component": "button_maximize",
                    onclick: move |_| {
                        win_maximize.toggle_maximized();
                    },
                }
                button {
                    "data-component": "button_close",
                    onclick: move |_| {
                        win_close.close();
                    },
                }
            }
        }
    }
}
