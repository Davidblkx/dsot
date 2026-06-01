use dioxus::prelude::*;

use super::routes::Routes;

#[component]
pub fn Layout() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Routes::HomeView,
                "Home"
            }
            Link {
                to: Routes::ConfigView,
                "Config"
            }
            Link {
                to: Routes::InboxView,
                "Inbox"
            }
        }

        Outlet::<Routes> {}
    }
}
