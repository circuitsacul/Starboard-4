use anyhow::Result;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::application_command::InteractionChannel;

use crate::interactions::commands::context::CommandCtx;
use crate::interactions::commands::permissions::manage_channels;
use crate::models::{AutoStarChannel, Guild};
use crate::{create_maybe, get_guild_id, unwrap_id};

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "create",
    desc = "Create an autostar channel.",
    dm_permission = false,
    default_permissions = "manage_channels"
)]
pub struct CreateAutoStarChannel {
    /// The name of the autostar channel.
    name: String,
    /// The channel to create an autostar channel in.
    #[command(channel_types = "guild_text guild_news")]
    channel: InteractionChannel,
}

impl CreateAutoStarChannel {
    pub async fn callback(self, ctx: CommandCtx) -> Result<()> {
        let guild_id = get_guild_id!(ctx);
        create_maybe!(Guild, &ctx.bot.pool, guild_id)?;

        AutoStarChannel::create(
            &ctx.bot.pool,
            &self.name,
            unwrap_id!(self.channel.id),
            guild_id,
        )
        .await?;

        ctx.respond_str("Created autostar channel in", false)
            .await?;
        Ok(())
    }
}
