use dioxus::prelude::*;

use super::model::{AddInboxValueState, InboxItemType};

#[component]
pub fn FormInputs(state: Store<AddInboxValueState>) -> Element {
    let selected = state.read().form_type.clone();

    let input = match selected {
        // TODO: use a native file/folder selector in order to get the actual path
        InboxItemType::File => {
            rsx! {
                div {
                    class: "form-group",
                    label { for: "file_input", "File or Directory" }
                    div {
                        class: "file-input-wrapper",
                        input {
                            id: "file_input",
                            type: "file",
                            multiple: true,
                            oninput: move |e| {
                                let files = e.files();
                                if files.len() == 0 {
                                    return;
                                } else if files.len() == 1 {
                                    if let Some(f) = files.get(0) {
                                        state.write().text = f.name();
                                    }
                                } else {
                                    let mut v = vec![];
                                    for i in files {
                                        v.push(i.name());
                                    }
                                    state.write().text = v.join(",");
                                }
                            },
                        }
                        span {
                           class: "selected_files",
                           "{state.read().text}"
                        }
                    }
                }
            }
        }
        InboxItemType::Artist => rsx! {},
        InboxItemType::Album => rsx! {},
        InboxItemType::Link => rsx! {},
        _ => rsx! {
            textarea {
                value: "{state.read().text}",
                placeholder: "Enter details here...",
                oninput: move |evt| state.write().text = evt.value()
            }
        },
    };

    rsx! {
        div {
            class: "form_input",
            {input}
        }
    }
}
