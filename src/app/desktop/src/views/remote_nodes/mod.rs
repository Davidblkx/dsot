use dioxus::prelude::*;

use crate::state::remote_nodes::use_address_book_reader;
use crate::widgets::views::View;

mod header;
mod list;
mod selected;

static CSS: Asset = asset!("/assets/styles/view/nodes.css");

#[component]
pub fn RemoteNodesView() -> Element {
    let trigger = use_address_book_reader();

    rsx! {
        View {
            name: "remote_nodes",
            css: CSS,

            header::RemoteNodesHeader {  }

            div {
                "data-component": "nodes_view",
                list::RemoteNodeList { trigger }
                selected::RemoteNodeSelected {  }
            }
        }
    }
}
