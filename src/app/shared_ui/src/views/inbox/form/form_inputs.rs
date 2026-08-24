use dioxus::prelude::*;

use crate::views::inbox::form::model::InboxFormStateStoreExt;

use super::{
    inputs::{AlbumInput, ArtistInput, FileInput, LinkInput, OtherInput},
    model::InboxItemType,
    state::InboxFormStore,
};

#[component]
pub fn FormInputs(platform_file_input: Option<Element>) -> Element {
    let selected = use_context::<InboxFormStore>().form_type()();

    let input = match selected {
        InboxItemType::File => {
            if let Some(input) = platform_file_input {
                input
            } else {
                rsx! {
                    FileInput {  }
                }
            }
        }
        InboxItemType::Artist => rsx! {
            ArtistInput {  }
        },
        InboxItemType::Album => rsx! {
            AlbumInput {  }
        },
        InboxItemType::Link => rsx! {
            LinkInput {  }
        },
        _ => rsx! {
            OtherInput {  }
        },
    };

    rsx! {
        div {
            class: "form_input",
            {input}
        }
    }
}
