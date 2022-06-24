use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::get_guild_id;
use crate::interactions::commands::context::CommandCtx;
use crate::models::AutoStarChannel;

#[derive(CreateCommand, CommandModel)]
#[command(name = "view", desc = "View your autostar channels.")]
pub struct ViewAutoStarChannels {
    /// The name of the autostar channel to view. Leave blank to show all.
    name: Option<String>,
}

impl ViewAutoStarChannels {
    pub async fn callback(self, ctx: CommandCtx) -> anyhow::Result<()> {
        let guild_id = get_guild_id!(ctx);

        if let Some(name) = &self.name {
            let asc = AutoStarChannel::get_by_name(&ctx.bot.pool, name, guild_id).await?;

            if let Some(asc) = asc {
                ctx.respond_str(&format!("{:?}", asc), false).await?;
            } else {
                ctx.respond_str("No autostar channels with that name were found.", true)
                    .await?;
            }
        } else {
            let asc = AutoStarChannel::list_by_guild(&ctx.bot.pool, guild_id).await?;

            let msg = match asc.len() {
                0 => ("This server has no autostar channels.".into(), true),
                _ => (format!("{:?}", asc), false),
            };

            ctx.respond_str(&msg.0, msg.1).await?;
        }

        Ok(())
    }
}
