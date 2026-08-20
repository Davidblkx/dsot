use dioxus::prelude::*;

use super::inputs::{AlbumInput, ArtistInput, FileInput, LinkInput, OtherInput};
use super::model::{AddInboxValueState, AddInboxValueStateStoreExt, InboxItemType};

#[component]
pub fn FormInputs(state: Store<AddInboxValueState>) -> Element {
    let selected = state.form_type()();

    let input = match selected {
        InboxItemType::File => rsx! { FileInput { state } },
        InboxItemType::Artist => rsx! { ArtistInput { state } },
        InboxItemType::Album => rsx! { AlbumInput { state } },
        InboxItemType::Link => rsx! { LinkInput { state } },
        _ => rsx! { OtherInput { state } },
    };

    rsx! {
        div {
            class: "form_input",
            {input}
        }
    }
}
