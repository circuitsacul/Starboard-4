use anyhow::Result;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::application_command::InteractionChannel;

use crate::interactions::commands::context::CommandCtx;
use crate::models::{AutoStarChannel, Guild};
use crate::{channel_is_textable, create_maybe, get_guild_id};

#[derive(CommandModel, CreateCommand)]
#[command(name = "create", desc = "Create an autostar channel.")]
pub struct CreateAutoStarChannel {
    /// The name of the autostar channel.
    name: String,
    /// The channel to create an autostar channel in.
    channel: InteractionChannel,
}

impl CreateAutoStarChannel {
    pub async fn callback(self, ctx: CommandCtx) -> Result<()> {
        channel_is_textable!(ctx, self.channel);
        let guild_id = get_guild_id!(ctx);
        create_maybe!(Guild, &ctx.bot.pool, guild_id)?;

        AutoStarChannel::create(
            &ctx.bot.pool,
            &self.name,
            self.channel.id.get().try_into().unwrap(),
            guild_id,
        )
        .await?;

        ctx.respond_str("Created autostar channel in", false)
            .await?;
        Ok(())
    }
}
