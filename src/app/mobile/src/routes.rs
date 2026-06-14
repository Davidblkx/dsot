use dioxus::prelude::*;

use super::layout::Layout;
use dsot_shared_ui::views::{ConfigView, HomeView, InboxView};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Routes {
    #[layout(Layout)]
    #[route("/")]
    HomeView,
    #[route("/config")]
    ConfigView,
    #[route("/inbox")]
    InboxView,
}
