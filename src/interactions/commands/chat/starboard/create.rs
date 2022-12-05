use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::application_command::InteractionChannel;

use crate::{
    constants,
    database::{validation, Guild, Starboard},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    map_dup_none, unwrap_id,
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "create", desc = "Create a starboard.")]
pub struct CreateStarboard {
    /// The name of the starboard.
    name: String,
    /// The channel to create a starboard in.
    #[command(channel_types = "guild_text")]
    channel: InteractionChannel,
}

impl CreateStarboard {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);
        let guild_id_i64 = unwrap_id!(guild_id);
        map_dup_none!(Guild::create(&ctx.bot.pool, unwrap_id!(guild_id)))?;
        let channel_id = unwrap_id!(self.channel.id);

        let count = Starboard::count_by_guild(&ctx.bot.pool, guild_id_i64).await?;
        if count >= constants::MAX_STARBOARDS {
            ctx.respond_str(
                &format!(
                    "You can only have up to {} starboards.",
                    constants::MAX_STARBOARDS
                ),
                true,
            )
            .await?;
            return Ok(());
        }

        let name = match validation::name::validate_name(&self.name) {
            Err(why) => {
                ctx.respond_str(&why, true).await?;
                return Ok(());
            }
            Ok(name) => name,
        };

        let ret = map_dup_none!(Starboard::create(
            &ctx.bot.pool,
            &name,
            channel_id,
            unwrap_id!(guild_id),
        ))?;

        if ret.is_none() {
            ctx.respond_str(
                &format!("A starboard with the name '{}' already exists.", name),
                true,
            )
            .await?;
        } else {
            ctx.bot.cache.guild_starboard_names.remove(&guild_id).await;
            ctx.bot
                .cache
                .guild_vote_emojis
                .remove(&unwrap_id!(guild_id));

            ctx.respond_str(
                &format!("Created starboard '{}' in <#{}>.", name, channel_id),
                false,
            )
            .await?;
        }

        Ok(())
    }
}
