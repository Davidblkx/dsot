use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons;

use dsot_lib::{dsot_model::InboxValue, uuid};
use dsot_shared_ui::widgets::icon::*;

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

#[derive(Debug, Clone, PartialEq, Default, Store)]
pub struct InboxFormState {
    pub item: InboxFormItem,
    pub form_type: InboxItemType,
    pub text: String,
    pub album_artist: String,
    pub album_year: Option<u32>,
}

impl InboxFormState {
    pub fn new(item: InboxFormItem) -> Self {
        Self {
            item,
            form_type: InboxItemType::default(),
            text: String::new(),
            album_artist: String::new(),
            album_year: None,
        }
    }

    pub fn into_inbox_item(&self) -> InboxValue {
        match self.form_type {
            InboxItemType::Album => InboxValue::Album {
                album: self.text.clone(),
                artist: self.album_artist.clone(),
                year: self.album_year.clone(),
            },
            InboxItemType::Artist => InboxValue::Artist(self.text.clone()),
            InboxItemType::File => InboxValue::File(self.text.clone()),
            InboxItemType::Link => InboxValue::Link(self.text.clone()),
            InboxItemType::Other => InboxValue::Other(self.text.clone()),
        }
    }

    pub fn from_inbox_item(&mut self, item: InboxValue) {
        match item {
            InboxValue::Album {
                album,
                artist,
                year,
            } => {
                self.form_type = InboxItemType::Album;
                self.text = album;
                self.album_artist = artist;
                self.album_year = year;
            }
            InboxValue::Artist(artist) => {
                self.form_type = InboxItemType::Artist;
                self.text = artist;
            }
            InboxValue::File(file) => {
                self.form_type = InboxItemType::File;
                self.text = file;
            }
            InboxValue::Link(link) => {
                self.form_type = InboxItemType::Link;
                self.text = link;
            }
            InboxValue::Other(other) => {
                self.form_type = InboxItemType::Other;
                self.text = other;
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum InboxFormItem {
    #[default]
    New,
    Edit(uuid::Uuid),
}
