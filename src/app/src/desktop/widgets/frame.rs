use dioxus::{
    desktop::{tao::window::ResizeDirection, use_window},
    prelude::*,
};

#[derive(Props, Debug, PartialEq, Clone)]
struct FrameProps {
    pub pos: Position,
    pub dir: ResizeDirection,
}

#[derive(Debug, PartialEq, Clone)]
enum Position {
    Left,
    Right,
    Top,
    Bottom,
    Corner,
}

fn calc_corner_name(dir: &ResizeDirection) -> &'static str {
    match dir {
        ResizeDirection::NorthEast => "top_right",
        ResizeDirection::NorthWest => "top_left",
        ResizeDirection::SouthEast => "bottom_right",
        ResizeDirection::SouthWest => "bottom_left",
        _ => "",
    }
}

fn calc_position(pos: &Position, dir: &ResizeDirection) -> &'static str {
    match (pos, dir) {
        (Position::Left, _) => "left",
        (Position::Right, _) => "right",
        (Position::Top, _) => "top",
        (Position::Bottom, _) => "bottom",
        (Position::Corner, _) => calc_corner_name(dir),
    }
}

#[component]
fn BaseFrame(props: FrameProps) -> Element {
    let win = use_window();
    let pos = calc_position(&props.pos, &props.dir);
    let dir = props.dir;

    rsx! {
        div {
            "data-component": "desktop_frame",
            "data-position": "{pos}",
            onmousedown: move |_| {
                if let Err(e) = win.drag_resize_window(dir) {
                    ::log::error!("Failed to drag/resize window: {e}");
                }
            }
        }
    }
}

static CSS: Asset = asset!("/assets/styles/desktop/widgets/frame.css");

#[component]
pub fn DesktopFrame() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

        BaseFrame {
            pos: Position::Corner,
            dir: ResizeDirection::NorthEast,
        }
        BaseFrame {
            pos: Position::Corner,
            dir: ResizeDirection::SouthWest,
        }
        BaseFrame {
            pos: Position::Corner,
            dir: ResizeDirection::NorthWest,
        }
        BaseFrame {
            pos: Position::Corner,
            dir: ResizeDirection::SouthEast,
        }
        BaseFrame {
            pos: Position::Top,
            dir: ResizeDirection::North,
        }
        BaseFrame {
            pos: Position::Bottom,
            dir: ResizeDirection::South,
        }
        BaseFrame {
            pos: Position::Left,
            dir: ResizeDirection::West,
        }
        BaseFrame {
            pos: Position::Right,
            dir: ResizeDirection::East,
        }
    }
}
