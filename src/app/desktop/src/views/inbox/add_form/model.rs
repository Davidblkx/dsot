use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons;
use dsot_shared_ui::widgets::icon::*;

#[derive(Debug, Clone, PartialEq, Default, Store)]
pub struct AddInboxValueState {
    pub form_type: InboxItemType,
    pub text: String,
    pub album_artist: String,
    pub album_year: Option<i64>,
}

impl AddInboxValueState {
    pub fn reset(&mut self) {
        self.form_type = InboxItemType::Other;
        self.text = String::new();
        self.album_artist = String::new();
        self.album_year = None;
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum InboxItemType {
    File,
    Artist,
    Album,
    Link,
    #[default]
    Other,
}

impl InboxItemType {
    pub fn get_name(&self) -> &'static str {
        match self {
            InboxItemType::File => "File",
            InboxItemType::Artist => "Artist",
            InboxItemType::Album => "Album",
            InboxItemType::Link => "Link",
            InboxItemType::Other => "Other",
        }
    }

    pub fn get_icon(&self) -> Element {
        match self {
            InboxItemType::File => icon(ld_icons::LdFileAudio),
            InboxItemType::Artist => icon(ld_icons::LdUser),
            InboxItemType::Album => icon(ld_icons::LdAlbum),
            InboxItemType::Link => icon(ld_icons::LdLink),
            InboxItemType::Other => icon(ld_icons::LdNotebook),
        }
    }

    pub fn get_icon_name(&self) -> (&'static str, Element) {
        (self.get_name(), self.get_icon())
    }
}
