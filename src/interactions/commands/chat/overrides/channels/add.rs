use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::{validation::mentions::textable_channel_ids, StarboardOverride},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    locale_func,
    utils::id_as_i64::GetI64,
};

locale_func!(overrides_channels_add);
locale_func!(overrides_channels_add_option_name);
locale_func!(overrides_channels_add_option_channels);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "add",
    desc = "Add channels to an override.",
    desc_localizations = "overrides_channels_add"
)]
pub struct AddOverrideChannels {
    /// The override to add channels to.
    #[command(
        autocomplete = true,
        rename = "override",
        desc_localizations = "overrides_channels_add_option_name"
    )]
    name: String,

    /// The channels to add.
    #[command(desc_localizations = "overrides_channels_add_option_channels")]
    channels: String,
}

impl AddOverrideChannels {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);
        let guild_id_i64 = guild_id.get_i64();
        let lang = ctx.user_lang();

        let ov = StarboardOverride::get(&ctx.bot.pool, guild_id_i64, &self.name).await?;
        if let Some(ov) = ov {
            let mut channel_ids = textable_channel_ids(&ctx.bot, guild_id, &self.channels).await?;
            channel_ids.extend(ov.channel_ids);
            let new_channels: Vec<_> = channel_ids.into_iter().collect();

            if let Err(why) = StarboardOverride::validate_channels(&new_channels) {
                ctx.respond_str(&why, true).await?;
                return Ok(());
            }
            let ret = StarboardOverride::set_channels(
                &ctx.bot.pool,
                guild_id_i64,
                &self.name,
                &new_channels,
            )
            .await?;

            if ret.is_some() {
                ctx.respond_str(&lang.overrides_channels_done(self.name), false)
                    .await?;
                return Ok(());
            }
        }

        ctx.respond_str(&lang.override_missing(self.name), true)
            .await?;
        Ok(())
    }
}
