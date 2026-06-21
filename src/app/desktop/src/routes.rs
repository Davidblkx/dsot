use dioxus::prelude::*;

use super::layout::Layout;
use super::views::{InboxView, RemoteView};
use dsot_shared_ui::views::{ConfigView, HomeView};

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
    #[route("/remote")]
    RemoteView,
}
