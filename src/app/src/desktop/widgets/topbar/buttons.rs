use dioxus::{desktop::use_window, prelude::*};

#[component]
pub fn Buttons() -> Element {
    let win_minimize = use_window();
    let win_maximize = win_minimize.clone();
    let win_close = win_minimize.clone();

    rsx! {
        div {
            "data-component": "button_group",
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
