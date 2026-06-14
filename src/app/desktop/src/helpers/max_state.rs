use dioxus::{
    desktop::{
        tao::event::{Event, WindowEvent},
        use_window, use_wry_event_handler,
    },
    prelude::*,
};

pub fn track_state() {
    let win = use_window();
    let start_state = win.is_maximized();
    let mut is_maximized = use_signal(|| start_state);

    use_wry_event_handler(move |event, _| {
        if let Event::WindowEvent {
            event: WindowEvent::Resized(..),
            ..
        } = event
        {
            is_maximized.set(win.is_maximized());
        }
    });

    use_effect(move || {
        let status = if is_maximized() {
            "maximized"
        } else {
            "normal"
        };
        let js_eval = document::eval(&format!(
            "document.documentElement.setAttribute('data-window-status', '{}');",
            status
        ));

        spawn(async move {
            match js_eval.await {
                Ok(_) => {}
                Err(e) => ::log::warn!("Failed to set window status: {}", e),
            }
        });
    });
}
