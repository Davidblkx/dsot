use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdRouter};

use crate::widgets::views::{Header, View};
use dsot_shared_ui::widgets::network::MyAddress;

static CSS: Asset = asset!("/assets/styles/view/remote.css");

#[component]
pub fn RemoteView() -> Element {
    let icon = rsx! {
        Icon {
            icon: LdRouter
        }
    };

    rsx! {
        View {
            name: "remote",
            css: CSS,

            Header {
                title: "Remote",
                icon: icon,
                MyAddress {  }
            }
        }
    }
}
