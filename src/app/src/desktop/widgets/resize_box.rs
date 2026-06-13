use dioxus::{
    desktop::{tao::window::ResizeDirection, use_window},
    prelude::*,
};

#[component]
pub fn ResizeBox() -> Element {
    let win = use_window();

    rsx! {
        div {
            "data-component": "resize_box",
            onclick: move |evt| {
                let dir = ResizeDirection::NorthEast;
                match win.drag_resize_window(dir) {
                    Ok(_) => {}
                    Err(_) => {}
                }
            }
        }
    }
}
