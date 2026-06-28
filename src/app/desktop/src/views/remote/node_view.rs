use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::{LdRouter, LdCopy, LdRefreshCw, LdSettings, LdTrash2, LdNetwork, LdActivity}};
use crate::state::remote::{RemoteStore, SelectedMachine, MachineStatus, SyncStatus, RemoteStateStoreExt};

#[component]
pub fn NodeView(trigger: Signal<i32>) -> Element {
    let mut state = use_context::<RemoteStore>();
    let selected = state.selected();
    let items = state.items();

    let selected_val = selected.read().clone();
    
    match selected_val {
        SelectedMachine::None => {
            rsx! {
                div {
                    "data-component": "node_view",
                    class: "node-view-empty",
                    div {
                        class: "empty-content",
                        Icon {
                            class: "empty-icon",
                            icon: LdRouter,
                        }
                        h3 { "No Device Selected" }
                        p { "Choose a remote device from the list on the left to configure connection settings and manage metadata synchronization." }
                    }
                }
            }
        }
        SelectedMachine::Machine(index) => {
            let items_ref = items.read();
            if index >= items_ref.len() {
                return rsx! {
                    div {
                        "data-component": "node_view",
                        class: "node-view-empty",
                        "Device not found"
                    }
                };
            }
            let item = items_ref[index].clone();
            let endpoint_str = item.id.to_string();

            let (status_text, status_badge_class) = match item.status {
                MachineStatus::Online(_) => ("Online", "badge-online"),
                MachineStatus::Offline => ("Offline", "badge-offline"),
            };

            let (sync_text, sync_badge_class) = match item.sync {
                SyncStatus::Disabled => ("Disabled", "badge-disabled"),
                SyncStatus::Waiting => ("Waiting", "badge-waiting"),
                SyncStatus::Pending => ("Pending", "badge-pending"),
                SyncStatus::Done => ("Synchronized", "badge-done"),
                SyncStatus::Syncing => ("Syncing...", "badge-syncing"),
                SyncStatus::Failure => ("Failed", "badge-failed"),
            };

            rsx! {
                div {
                    "data-component": "node_view",
                    class: "node-view-details",
                    
                    div {
                        class: "details-header",
                        div {
                            class: "title-row",
                            h2 { "{item.name}" }
                            div {
                                class: "badges",
                                span {
                                    class: "badge {status_badge_class}",
                                    "{status_text}"
                                }
                                span {
                                    class: "badge {sync_badge_class}",
                                    "{sync_text}"
                                }
                            }
                        }
                        if !item.desc.is_empty() {
                            p {
                                class: "description",
                                "{item.desc}"
                            }
                        }
                    }

                    div {
                        class: "details-body",
                        
                        div {
                            class: "card network-card",
                            h3 { 
                                Icon { icon: LdNetwork }
                                "Network Properties" 
                            }
                            div {
                                class: "property-row",
                                label { "Endpoint ID (Iroh Address)" }
                                div {
                                    class: "address-container",
                                    code { "{endpoint_str}" }
                                    button {
                                        class: "btn-copy",
                                        title: "Copy Address to Clipboard",
                                        onclick: move |_| {
                                            let js = format!("navigator.clipboard.writeText('{}')", endpoint_str);
                                            let _ = document::eval(&js);
                                        },
                                        Icon { icon: LdCopy }
                                    }
                                }
                            }
                        }

                        div {
                            class: "card sync-card",
                            h3 { 
                                Icon { icon: LdActivity }
                                "Synchronization" 
                            }
                            div {
                                class: "sync-stats",
                                div {
                                    class: "stat-item",
                                    span { class: "stat-label", "Status" }
                                    span { class: "stat-val", "{sync_text}" }
                                }
                                div {
                                    class: "stat-item",
                                    span { class: "stat-label", "Last Synced" }
                                    span { class: "stat-val", "Never" }
                                }
                            }
                        }
                    }

                    div {
                        class: "details-actions",
                        button {
                            class: "btn-action btn-primary",
                            Icon { icon: LdRefreshCw }
                            "Sync Now"
                        }
                        button {
                            class: "btn-action btn-secondary",
                            Icon { icon: LdSettings }
                            "Configure"
                        }
                        div { class: "spacer" }
                        button {
                            class: "btn-action btn-danger",
                            onclick: move |_| {
                                // Clear selected and delete item
                                let mut state_write = state.write();
                                state_write.items.remove(index);
                                state_write.selected = SelectedMachine::None;
                            },
                            Icon { icon: LdTrash2 }
                            "Forget Device"
                        }
                    }
                }
            }
        }
    }
}
