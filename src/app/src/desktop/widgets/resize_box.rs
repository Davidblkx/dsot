use dioxus::{
    desktop::{tao::window::ResizeDirection, use_window},
    prelude::*,
};

fn determine_resize_direction(x: f64, y: f64, threshold: f64) -> ResizeDirection {
    let diff = x - y;
    if diff.abs() < threshold {
        ResizeDirection::SouthEast
    } else if diff > 0.0 {
        ResizeDirection::East
    } else {
        ResizeDirection::South
    }
}

#[component]
pub fn ResizeBox() -> Element {
    let win = use_window();

    rsx! {
        div {
            "data-component": "resize_box",
            style: "position: fixed; bottom: 0; right: 0; z-index: 99999; width: 50px; height: 50px; background: red;",
            onmousedown: move |evt| {
                let coords = evt.data().element_coordinates();
                let dir = determine_resize_direction(coords.x, coords.y, 4.0);
                if let Err(e) = win.drag_resize_window(dir) {
                    log::error!("Failed to drag resize window: {:?}", e);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_resize_direction() {
        // Near the diagonal (within 4.0 threshold) should be SouthEast
        assert_eq!(
            determine_resize_direction(10.0, 10.0, 4.0),
            ResizeDirection::SouthEast
        );
        assert_eq!(
            determine_resize_direction(10.0, 12.0, 4.0),
            ResizeDirection::SouthEast
        );
        assert_eq!(
            determine_resize_direction(12.0, 10.0, 4.0),
            ResizeDirection::SouthEast
        );

        // Above the diagonal (x > y + threshold) should be East
        assert_eq!(
            determine_resize_direction(15.0, 10.0, 4.0),
            ResizeDirection::East
        );

        // Below the diagonal (y > x + threshold) should be South
        assert_eq!(
            determine_resize_direction(10.0, 15.0, 4.0),
            ResizeDirection::South
        );
    }
}
