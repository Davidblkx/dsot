use crate::cmd::error::AppResult;
use crate::cmd::infra::{AppCommand, AppCommandContext};

use dsot_runtime::{InboxInsertValue, InboxItems};

pub struct AddCommand;
declare_arg_string!(TitleArg, "title", "Song title", 't');
declare_arg_string!(ArtistArg, "artist", "Artist name", 'a');
declare_arg_string!(AlbumArg, "album", "Album name", 'r');
declare_arg_path!(FileArg, "file", "Path to file", 'f');
declare_arg_string!(
    NotesArg,
    "notes",
    "Notes or any extra info about entry",
    'n'
);

impl AppCommand for AddCommand {
    fn name() -> &'static str {
        "add"
    }

    fn build() -> clap::Command {
        clap::Command::new(AddCommand::name())
            .about("Add new item to inbox")
            .arg(TitleArg::build())
            .arg(ArtistArg::build())
            .arg(AlbumArg::build())
            .arg(FileArg::build())
            .arg(NotesArg::build())
    }

    async fn execute(runtime: &dsot_runtime::Runtime, context: AppCommandContext) -> AppResult<()> {
        let mut to_add = InboxInsertValue::new();

        if let Some(v) = TitleArg::get(&context.args) {
            to_add.set_title(v);
        }

        if let Some(v) = ArtistArg::get(&context.args) {
            to_add.set_artist(v);
        }

        if let Some(v) = AlbumArg::get(&context.args) {
            to_add.set_album(v);
        }

        if let Some(v) = FileArg::get(&context.args) {
            if let Some(v) = v.to_str() {
                to_add.set_file(v);
            } else {
                log::trace!("Invalid path: {}", v.display());
            }
        }

        if let Some(v) = NotesArg::get(&context.args) {
            to_add.set_extra_info(v);
        }

        if to_add.has_value() {
            runtime.add_inbox(to_add).await?;
        } else {
            log::warn!("Inbox entry has no value, nothing will be added");
        }

        Ok(())
    }
}
