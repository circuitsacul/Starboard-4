use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_mention::Mention;
use twilight_model::application::interaction::application_command::InteractionChannel;

use crate::{
    constants,
    core::premium::is_premium::is_guild_premium,
    database::{validation, AutoStarChannel, DbGuild},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    locale_func,
    utils::id_as_i64::GetI64,
};

locale_func!(autostar_create);
locale_func!(autostar_create_option_name);
locale_func!(autostar_create_option_channel);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "create",
    desc = "Create an autostar channel.",
    desc_localizations = "autostar_create"
)]
pub struct CreateAutoStarChannel {
    /// The name of the autostar channel.
    #[command(desc_localizations = "autostar_create_option_name")]
    name: String,
    /// The channel to create an autostar channel in.
    #[command(
        channel_types = r#"
            guild_text
            guild_voice
            guild_stage_voice
            guild_announcement
            announcement_thread
            public_thread
            private_thread
            guild_forum
        "#,
        desc_localizations = "autostar_create_option_channel"
    )]
    channel: InteractionChannel,
}

impl CreateAutoStarChannel {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();
        let channel_id = self.channel.id.get_i64();

        DbGuild::create(&ctx.bot.pool, guild_id).await?;

        let name = match validation::name::validate_name(&self.name) {
            Err(why) => {
                ctx.respond_str(&why, true).await?;
                return Ok(());
            }
            Ok(name) => name,
        };

        let count = AutoStarChannel::count_by_guild(&ctx.bot.pool, guild_id).await?;
        let limit = if is_guild_premium(&ctx.bot, guild_id, true).await? {
            constants::MAX_PREM_AUTOSTAR
        } else {
            constants::MAX_AUTOSTAR
        };
        if count >= limit {
            ctx.respond_str(
                &ctx.user_lang()
                    .autostar_create_limit_reached(limit, constants::MAX_PREM_AUTOSTAR),
                true,
            )
            .await?;
            return Ok(());
        }

        let ret = AutoStarChannel::create(&ctx.bot.pool, &name, channel_id, guild_id).await?;

        if ret.is_none() {
            ctx.respond_str(&ctx.user_lang().autostar_channel_already_exists(name), true)
                .await?;
        } else {
            ctx.bot.cache.autostar_channel_ids.insert(self.channel.id);

            ctx.respond_str(
                &ctx.user_lang()
                    .autostar_create_success(self.channel.id.mention(), name),
                false,
            )
            .await?;
        }

        Ok(())
    }
}
