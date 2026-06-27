use dioxus::prelude::*;

use crate::state::remote::use_remote_machines;
use crate::widgets::views::View;

mod header;

static CSS: Asset = asset!("/assets/styles/view/remote.css");

#[component]
pub fn RemoteView() -> Element {
    let _trigger = use_remote_machines();

    rsx! {
        View {
            name: "remote",
            css: CSS,

            header::RemoteHeader {  }

            div {
                "data-component": "remote_machines",

            }
        }
    }
}
