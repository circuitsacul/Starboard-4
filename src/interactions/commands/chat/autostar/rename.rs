use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{get_guild_id, interactions::commands::context::CommandCtx, models::AutoStarChannel};

#[derive(CreateCommand, CommandModel)]
#[command(name = "rename", desc = "Rename an autostar channel.")]
pub struct RenameAutoStarChannel {
    /// The current name of the autostar channel.
    #[command(rename = "current-name")]
    current_name: String,
    /// The new name for the autostar channel.
    #[command(rename = "new-name")]
    new_name: String,
}

impl RenameAutoStarChannel {
    pub async fn callback(self, ctx: CommandCtx) -> anyhow::Result<()> {
        let guild_id = get_guild_id!(ctx);

        let ret =
            AutoStarChannel::rename(&ctx.bot.pool, &self.current_name, guild_id, &self.new_name)
                .await?;
        if ret.is_none() {
            ctx.respond_str("No autostar channel with that name was found.", true)
                .await?;
        } else {
            ctx.respond_str("Renamed the autostar channel.", false)
                .await?;
        }

        Ok(())
    }
}
