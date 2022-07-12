pub mod embed;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::interactions::commands::context::CommandCtx;

#[derive(CommandModel, CreateCommand)]
#[command(name = "edit", desc = "Edit a starboard")]
pub enum EditStarboard {
    #[command(name = "embed")]
    Embed(embed::EditEmbedStyle),
}

impl EditStarboard {
    pub async fn call_callback(self, ctx: CommandCtx) -> anyhow::Result<()> {
        match self {
            Self::Embed(cmd) => cmd.callback(ctx).await,
        }
    }
}
