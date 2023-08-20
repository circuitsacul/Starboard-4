use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::{validation::mentions::textable_channel_ids, StarboardOverride},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    locale_func,
    utils::id_as_i64::GetI64,
};

locale_func!(overrides_channels_remove);
locale_func!(overrides_channels_remove_option_name);
locale_func!(overrides_channels_remove_option_channels);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "remove",
    desc = "Remove channels from an override.",
    desc_localizations = "overrides_channels_remove"
)]
pub struct RemoveOverrideChannels {
    /// The override to remove channels from.
    #[command(
        autocomplete = true,
        rename = "override",
        desc_localizations = "overrides_channels_remove_option_name"
    )]
    name: String,

    /// The channels to remove.
    #[command(desc_localizations = "overrides_channels_remove_option_channels")]
    channels: String,
}

impl RemoveOverrideChannels {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);
        let guild_id_i64 = guild_id.get_i64();
        let lang = ctx.user_lang();

        let ov = StarboardOverride::get(&ctx.bot.pool, guild_id_i64, &self.name).await?;
        if let Some(ov) = ov {
            let to_remove = textable_channel_ids(&ctx.bot, guild_id, &self.channels).await?;
            let channel_ids: Vec<_> = ov
                .channel_ids
                .iter()
                .copied()
                .filter(|id| !to_remove.contains(id))
                .collect();

            let ret = StarboardOverride::set_channels(
                &ctx.bot.pool,
                guild_id_i64,
                &self.name,
                &channel_ids,
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
