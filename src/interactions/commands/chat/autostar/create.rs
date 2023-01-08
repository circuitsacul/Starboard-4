use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::application_command::InteractionChannel;

use crate::{
    constants,
    database::{validation, AutoStarChannel, Guild},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    map_dup_none,
    utils::id_as_i64::GetI64,
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "create", desc = "Create an autostar channel.")]
pub struct CreateAutoStarChannel {
    /// The name of the autostar channel.
    name: String,
    /// The channel to create an autostar channel in.
    #[command(channel_types = "guild_text")]
    channel: InteractionChannel,
}

impl CreateAutoStarChannel {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);
        let guild_id_i64 = guild_id.get_i64();
        map_dup_none!(Guild::create(&ctx.bot.pool, guild_id_i64))?;
        let channel_id = self.channel.id.get_i64();

        let name = match validation::name::validate_name(&self.name) {
            Err(why) => {
                ctx.respond_str(&why, true).await?;
                return Ok(());
            }
            Ok(name) => name,
        };

        let count = AutoStarChannel::count_by_guild(&ctx.bot.pool, guild_id_i64).await?;
        if count >= constants::MAX_AUTOSTAR {
            ctx.respond_str(
                &format!(
                    "You can only have up to {} autostar channels.",
                    constants::MAX_AUTOSTAR
                ),
                true,
            )
            .await?;
            return Ok(());
        }

        let ret = map_dup_none!(AutoStarChannel::create(
            &ctx.bot.pool,
            &name,
            channel_id,
            guild_id_i64,
        ))?;

        if ret.is_none() {
            ctx.respond_str(
                &format!("An autostar channel with the name '{name}' already exists."),
                true,
            )
            .await?;
        } else {
            ctx.bot.cache.autostar_channel_ids.insert(self.channel.id);

            ctx.respond_str(
                &format!("Created autostar channel '{name}' in <#{channel_id}>."),
                false,
            )
            .await?;
        }

        Ok(())
    }
}
