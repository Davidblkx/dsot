use dioxus::prelude::*;

use super::layout::Layout;
use crate::views::devices::DevicesView;
use crate::views::inbox::InboxView;
use crate::views::inbox::add_form::AddInboxValue as AddInboxView;
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
    #[route("/inbox/add")]
    AddInboxView,
    #[route("/devices")]
    DevicesView,
}
