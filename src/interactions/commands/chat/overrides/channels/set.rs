use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::{validation::mentions::textable_channel_ids, StarboardOverride},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    locale_func,
    utils::id_as_i64::GetI64,
};

locale_func!(overrides_channels_set);
locale_func!(overrides_channels_set_option_name);
locale_func!(overrides_channels_set_option_channels);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "set",
    desc = "Set the channels that an override affects.",
    desc_localizations = "overrides_channels_set"
)]
pub struct SetOverrideChannels {
    /// The override to set the channels for.
    #[command(
        autocomplete = true,
        rename = "override",
        desc_localizations = "overrides_channels_set_option_name"
    )]
    name: String,

    /// A list of channels that the override should affect. Use "none" to
    /// remove all.
    #[command(desc_localizations = "overrides_channels_set_option_channels")]
    channels: String,
}

impl SetOverrideChannels {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);
        let guild_id_i64 = guild_id.get_i64();
        let lang = ctx.user_lang();

        let channel_ids: Vec<_> = textable_channel_ids(&ctx.bot, guild_id, &self.channels)
            .await?
            .into_iter()
            .collect();
        if let Err(why) = StarboardOverride::validate_channels(&channel_ids) {
            ctx.respond_str(&why, true).await?;
            return Ok(());
        }
        let ov =
            StarboardOverride::set_channels(&ctx.bot.pool, guild_id_i64, &self.name, &channel_ids)
                .await?;

        if ov.is_none() {
            ctx.respond_str(&lang.override_missing(self.name), true)
                .await?;
        } else {
            ctx.respond_str(&lang.overrides_channels_done(self.name), false)
                .await?;
        }
        Ok(())
    }
}
