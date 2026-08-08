use dioxus::prelude::*;
use dsot_lib::DsotCore;
use dsot_lib::dsot_model::InboxStatus;
use dsot_lib::state::inbox::InboxFilterOperations;
use dsot_shared_ui::sink::*;

#[component]
pub fn InboxFilter() -> Element {
    let core = use_context::<DsotCore>();
    let filter = use_receiver(core.state.inbox.table.filter.clone());

    let status = use_memo(move || {
        filter
            .read()
            .status
            .as_ref()
            .map_or("none".to_string(), |s| s.as_db_str().to_string())
    });

    let check_status = move |v: &str| v == status();

    rsx! {
        div {
            class: "filter",
            select {
                onchange: move |ev| core.state.inbox.table.update_status(InboxStatus::from_db_str(&ev.value()).ok()),
                option {
                    value: "none",
                    selected: check_status("none"),
                    "All Status"
                },
                option {
                    value: "Pending",
                    selected: check_status("Pending"),
                    "Pending"
                },
                option {
                    value: "Resolved",
                    selected: check_status("Resolved"),
                    "Completed"
                },
                option {
                    value: "Failed",
                    selected: check_status("Failed"),
                    "Canceled"
                }
            }
        }
    }
}
