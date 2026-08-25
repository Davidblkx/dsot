use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdHardDrive};
use dsot_shared_ui::views::devices::{DEVICES_CSS, address::AppAddress};

use crate::widgets::views::{Header, View};

const CSS: Asset = asset!("/assets/styles/view/devices.css");
const COMBINED_CSS: &[Asset] = &[CSS, DEVICES_CSS];

#[component]
pub fn DevicesView() -> Element {
    rsx! {
        View {
            name: "devices",
            css: COMBINED_CSS,
            Header {
                title: "Devices",
                icon: rsx! {
                    Icon {
                        icon: LdHardDrive
                    }
                }
            }

            AppAddress { }
        }
    }
}
