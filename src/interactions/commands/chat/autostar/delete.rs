use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::get_guild_id;
use crate::interactions::commands::context::CommandCtx;
use crate::models::AutoStarChannel;

#[derive(CreateCommand, CommandModel)]
#[command(name = "delete", desc = "Delete an autostar channel.")]
pub struct DeleteAutoStarChannel {
    /// The name of the autostar channel to delete.
    name: String,
}

impl DeleteAutoStarChannel {
    pub async fn callback(self, ctx: CommandCtx) -> anyhow::Result<()> {
        let guild_id = get_guild_id!(ctx);
        let ret = AutoStarChannel::delete(&ctx.bot.pool, &self.name, guild_id).await?;
        if ret.rows_affected() == 0 {
            ctx.respond_str("No autostar channel with that name was found.", true)
                .await?;
        } else {
            ctx.respond_str(&format!("Deleted autostar channel '{}'.", self.name), false)
                .await?;
        }
        Ok(())
    }
}
