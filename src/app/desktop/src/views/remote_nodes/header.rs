use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdRouter};
use dsot_shared_ui::widgets::network::MyAddress;

use crate::widgets::views::Header;

#[component]
pub fn RemoteNodesHeader() -> Element {
    let icon = rsx! {
        Icon {
            icon: LdRouter
        }
    };

    rsx! {
        Header {
            title: "Remote Nodes",
            icon: icon,
            MyAddress {  }
        }
    }
}
