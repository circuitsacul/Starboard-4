use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    get_guild_id, interactions::commands::context::CommandCtx, map_dup_none,
    models::AutoStarChannel,
};

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

        let ret = map_dup_none!(AutoStarChannel::rename(
            &ctx.bot.pool,
            &self.current_name,
            guild_id,
            &self.new_name
        ))?;

        match ret {
            None => {
                ctx.respond_str("An autostar channel with that name already exists.", true)
                    .await?
            }
            Some(None) => {
                ctx.respond_str("No autostar channel with that name was found.", true)
                    .await?
            }
            Some(Some(_)) => {
                ctx.respond_str("Renamed the autostar channel.", false)
                    .await?
            }
        };

        Ok(())
    }
}
