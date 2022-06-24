use anyhow::Result;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::application_command::InteractionChannel;

use crate::interactions::commands::context::CommandCtx;
use crate::models::{AutoStarChannel, Guild};
use crate::{create_maybe, get_guild_id, unwrap_id};

#[derive(CommandModel, CreateCommand)]
#[command(name = "create", desc = "Create an autostar channel.")]
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

        let channel_id = unwrap_id!(self.channel.id);

        AutoStarChannel::create(&ctx.bot.pool, &self.name, channel_id, guild_id).await?;

        ctx.respond_str(
            &format!(
                "Created autostar channel '{}' in <#{}>.",
                self.name, channel_id
            ),
            false,
        )
        .await?;
        Ok(())
    }
}
