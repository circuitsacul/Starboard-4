use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::application_command::InteractionChannel;

use crate::database::{validation, AutoStarChannel, Guild};
use crate::interactions::commands::context::CommandCtx;
use crate::{get_guild_id, map_dup_none, unwrap_id};

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
    pub async fn callback(self, ctx: CommandCtx) -> anyhow::Result<()> {
        let guild_id = get_guild_id!(ctx);
        map_dup_none!(Guild::create(&ctx.bot.pool, unwrap_id!(guild_id)))?;
        let channel_id = unwrap_id!(self.channel.id);

        let name = match validation::name::validate_name(&self.name) {
            Err(why) => {
                ctx.respond_str(&why, true).await?;
                return Ok(());
            }
            Ok(name) => name,
        };

        let ret = map_dup_none!(AutoStarChannel::create(
            &ctx.bot.pool,
            &name,
            channel_id,
            unwrap_id!(guild_id),
        ))?;

        if ret.is_none() {
            ctx.respond_str(
                &format!(
                    "An autostar channel with the name '{}' already exists.",
                    name
                ),
                true,
            )
            .await?;
        } else {
            ctx.bot.cache.autostar_channel_ids.insert(self.channel.id);

            ctx.respond_str(
                &format!("Created autostar channel '{}' in <#{}>.", name, channel_id),
                false,
            )
            .await?;
        }

        Ok(())
    }
}
