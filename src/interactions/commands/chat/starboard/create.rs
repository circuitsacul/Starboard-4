use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::application_command::InteractionChannel;

use crate::{
    constants,
    database::{validation, Guild, Starboard},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    map_dup_none,
    utils::id_as_i64::GetI64,
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "create", desc = "Create a starboard.")]
pub struct CreateStarboard {
    /// The name of the starboard.
    name: String,
    /// The channel to create a starboard in.
    #[command(channel_types = r#"
            guild_text
            guild_voice
            guild_announcement
            announcement_thread
            public_thread
            private_thread
        "#)]
    channel: InteractionChannel,
}

impl CreateStarboard {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);
        let guild_id_i64 = guild_id.get_i64();
        map_dup_none!(Guild::create(&ctx.bot.pool, guild_id_i64))?;
        let channel_id = self.channel.id.get_i64();

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
            guild_id.get_i64(),
        ))?;

        if ret.is_none() {
            ctx.respond_str(
                &format!("A starboard with the name '{name}' already exists."),
                true,
            )
            .await?;
        } else {
            ctx.bot.cache.guild_vote_emojis.remove(&guild_id_i64);

            ctx.respond_str(
                &format!("Created starboard '{name}' in <#{channel_id}>."),
                false,
            )
            .await?;
        }

        Ok(())
    }
}
