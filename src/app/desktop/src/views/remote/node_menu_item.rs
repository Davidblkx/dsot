use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::{LdCheck, LdRefreshCw, LdTriangleAlert}};
use crate::state::remote::{RemoteStore, SelectedMachine, MachineStatus, SyncStatus, RemoteStateStoreExt};

#[component]
pub fn NodeMenuItem(index: usize, trigger: Signal<i32>) -> Element {
    let state = use_context::<RemoteStore>();
    let items = state.items();
    let item = items.read()[index].clone();

    let selected = state.selected();
    let is_selected = match selected.read().clone() {
        SelectedMachine::Machine(idx) => idx == index,
        SelectedMachine::None => false,
    };

    let status_class = match item.status {
        MachineStatus::Online(_) => "status-online",
        MachineStatus::Offline => "status-offline",
    };

    let sync_icon = match item.sync {
        SyncStatus::InSync => rsx! { Icon { class: "sync-icon in-sync", icon: LdCheck } },
        SyncStatus::Failure => rsx! { Icon { class: "sync-icon failure", icon: LdTriangleAlert } },
        SyncStatus::Pending => rsx! { Icon { class: "sync-icon pending", icon: LdRefreshCw } },
        SyncStatus::Disabled => rsx! {},
    };

    rsx! {
        div {
            class: if is_selected { "node-menu-item selected" } else { "node-menu-item" },
            onclick: move |_| {
                state.selected().set(SelectedMachine::Machine(index));
            },
            span {
                class: "status-dot {status_class}"
            }
            div {
                class: "node-info",
                span {
                    class: "node-name",
                    "{item.name}"
                }
                span {
                    class: "node-desc",
                    "{item.desc}"
                }
            }
            span {
                class: "node-sync",
                {sync_icon}
            }
        }
    }
}
