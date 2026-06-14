use dioxus::{desktop::use_window, prelude::*};
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdChevronDown, LdChevronsUpDown, LdX};

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
                Icon {
                    icon: LdChevronDown
                }
            }
            button {
                "data-component": "button_maximize",
                onclick: move |_| {
                    win_maximize.toggle_maximized();
                },
                Icon {
                    icon: LdChevronsUpDown
                }
            }
            button {
                "data-component": "button_close",
                onclick: move |_| {
                    win_close.close();
                },
                Icon {
                    icon: LdX
                }
            }
        }
    }
}
