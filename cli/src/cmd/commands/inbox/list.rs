use crate::cmd::error::AppResult;
use crate::cmd::infra::{AppCommand, AppCommandContext};

use dsot_core::model::entities::inbox::Inbox;
use dsot_runtime::InboxItems;

declare_arg_number!(SizeArg(i64), "size", "Number of entries to show", 's');
declare_arg_number!(OffsetArg(i64), "offset", "Number of entries to skip", 'o');

pub struct ListCommand;

impl AppCommand for ListCommand {
    fn name() -> &'static str {
        "list"
    }

    fn build() -> clap::Command {
        clap::Command::new(ListCommand::name())
            .arg(SizeArg::build())
            .arg(OffsetArg::build())
    }

    async fn execute(runtime: &dsot_runtime::Runtime, context: AppCommandContext) -> AppResult<()> {
        let length = SizeArg::get(&context.args).unwrap_or(&10);
        let skip = OffsetArg::get(&context.args).unwrap_or(&0);

        let total = runtime.count_inbox().await?;
        let items = if total > 0 {
            runtime.list_inbox(*length, *skip).await?
        } else {
            Vec::new()
        };

        let mut index = *skip;
        for i in items {
            index += 1;
            context.print_message(get_display(&i, index, total));
        }

        Ok(())
    }
}

fn get_display(inbox: &Inbox, index: i64, total: i64) -> String {
    let info = if let (Some(title), Some(artist), Some(album)) =
        (&inbox.title, &inbox.artist, &inbox.album)
    {
        format!("{} by {} [{}]", title, artist, album)
    } else if let Some(title) = &inbox.title {
        format!(
            "{} by {}",
            title,
            inbox.artist.as_deref().unwrap_or("Unknown Artist")
        )
    } else if let Some(artist) = &inbox.artist {
        format!("artist: {}", artist)
    } else if let Some(album) = &inbox.album {
        format!("album: {}", album)
    } else {
        "".to_string()
    };

    let file = if info.len() > 0 {
        if let Some(file) = &inbox.file {
            format!(" [{}]", file)
        } else {
            "".to_string()
        }
    } else {
        inbox.file.as_deref().unwrap_or("").to_string()
    };

    let notes = if let Some(extra_info) = &inbox.extra_info {
        format!(" notes: {}", extra_info)
    } else {
        "".to_string()
    };

    vec![
        format!(
            "{}/{} [{}] ",
            index,
            total,
            inbox.id.to_string().replace("-", "")
        ),
        info,
        file,
        notes,
    ]
    .join("")
}
