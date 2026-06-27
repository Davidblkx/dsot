use dioxus::prelude::*;

use crate::state::remote::use_remote_machines;
use crate::widgets::views::View;

mod add_address_button;
mod address_editor;
mod header;
mod node_list;
mod node_menu_item;
mod node_view;

static CSS: Asset = asset!("/assets/styles/view/remote.css");

#[component]
pub fn RemoteView() -> Element {
    let trigger = use_remote_machines();

    rsx! {
        View {
            name: "remote",
            css: CSS,

            header::RemoteHeader {  }

            div {
                "data-component": "remote_machines",
                node_list::NodeList { trigger }
            }
        }
    }
}
