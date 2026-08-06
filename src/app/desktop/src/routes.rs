use dioxus_free_icons::icons::ld_icons::{LdHome, LdInbox};
use dsot_shared_ui::views::{ConfigView, HomeView};

use super::layout::Layout;
use crate::views::inbox::InboxView;

dsot_shared_ui::declare_router_menu!(Routes {
    layout: Layout,
    "/" => HomeView menu: LdHome "Home",
    "/config" => ConfigView,
    "/inbox" => InboxView menu: LdInbox "Inbox",
});
