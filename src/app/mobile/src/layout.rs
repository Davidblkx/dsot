use dioxus::prelude::*;

use super::routes::Routes;

#[component]
pub fn Layout() -> Element {
    rsx! {
        div {
            id: "navbar",
            style: "grid-area: footer !important;",
            Link {
                to: Routes::HomeView,
                "Home"
            }
            Link {
                to: Routes::DevicesView,
                "Devices"
            }
            Link {
                to: Routes::InboxView,
                "Inbox"
            }
        }

        Outlet::<Routes> {}
    }
}
