use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons;

use dsot_lib::dsot_model::InboxValue;
use dsot_shared_ui::widgets::icon::*;

#[derive(Debug, Clone, PartialEq, Default, Store)]
pub struct AddInboxValueState {
    pub form_type: InboxItemType,
    pub text: String,
    pub album_artist: String,
    pub album_year: Option<u32>,
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

impl Into<InboxValue> for AddInboxValueState {
    fn into(self) -> InboxValue {
        match self.form_type {
            InboxItemType::Album => InboxValue::Album {
                album: self.text,
                artist: self.album_artist,
                year: self.album_year,
            },
            InboxItemType::Artist => InboxValue::Artist(self.text),
            InboxItemType::File => InboxValue::File(self.text),
            InboxItemType::Link => InboxValue::Link(self.text),
            InboxItemType::Other => InboxValue::Other(self.text),
        }
    }
}
