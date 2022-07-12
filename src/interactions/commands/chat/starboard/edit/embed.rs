use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::interactions::commands::context::CommandCtx;

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "embed",
    desc = "Edit the style of the embeds sent to your starboard."
)]
pub struct EditEmbedStyle {}

impl EditEmbedStyle {
    pub async fn callback(self, mut ctx: CommandCtx) -> anyhow::Result<()> {
        ctx.respond_str("hello, world", false).await?;
        Ok(())
    }
}
