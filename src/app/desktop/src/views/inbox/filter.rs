use dioxus::prelude::*;
use dsot_lib::DsotCore;

#[component]
pub fn InboxFilter() -> Element {
    let core = use_context::<DsotCore>();

    rsx! {
        div {
            class: "filter",
            select {
                value: "pending",
                option {
                    value: "none",
                    label: "All Status"
                },
                option {
                    value: "pending",
                    label: "Pending"
                },
                option {
                    value: "completed",
                    label: "Completed"
                },
                option {
                    value: "failed",
                    label: "Failed"
                }
            }
        }
    }
}
