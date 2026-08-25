use dioxus_free_icons::icons::ld_icons::{LdCog, LdHome, LdInbox, LdRouter};
use dsot_shared_ui::views::{ConfigView, HomeView};

use super::layout::Layout;
use crate::views::{devices::DevicesView, inbox::InboxView};

dsot_shared_ui::declare_router_menu!(Routes {
    layout: Layout,
    "/" => HomeView menu: LdHome "Home",
    "/inbox" => InboxView menu: LdInbox "Inbox",
    "/devices" => DevicesView menu: LdRouter "Devices",
    "/config" => ConfigView menu: LdCog "Config",
});
