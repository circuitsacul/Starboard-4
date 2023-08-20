use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::{
        models::{
            autostar_channel_filter_group::AutostarChannelFilterGroup, filter_group::FilterGroup,
        },
        AutoStarChannel,
    },
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    locale_func,
    utils::id_as_i64::GetI64,
};

locale_func!(autostar_filters_remove);
locale_func!(autostar_filters_remove_option_autostar_channel);
locale_func!(autostar_filters_remove_option_filter_group);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "remove",
    desc = "Remove a filter group from an autostar channel.",
    desc_localizations = "autostar_filters_remove"
)]
pub struct Remove {
    /// The autostar channel to remove the filter group from.
    #[command(
        autocomplete = true,
        rename = "autostar-channel",
        desc_localizations = "autostar_filters_remove_option_autostar_channel"
    )]
    autostar_channel: String,
    /// The filter group to remove from the autostar channel.
    #[command(
        autocomplete = true,
        rename = "filter-group",
        desc_localizations = "autostar_filters_remove_option_filter_group"
    )]
    filter_group: String,
}

impl Remove {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();

        let Some(group) = FilterGroup::get_by_name(
            &ctx.bot.pool, guild_id, &self.filter_group
        ).await? else {
            ctx.respond_str(
                &ctx.user_lang().filter_group_missing(self.filter_group),
                true,
            ).await?;
            return Ok(());
        };

        let Some(asc) = AutoStarChannel::get_by_name(
            &ctx.bot.pool, &self.autostar_channel, guild_id
        ).await? else {
            ctx.respond_str(
                &ctx.user_lang().autostar_channel_missing(self.autostar_channel),
                true,
            ).await?;
            return Ok(());
        };

        let ret = AutostarChannelFilterGroup::delete(&ctx.bot.pool, group.id, asc.id).await?;

        if ret.is_some() {
            ctx.respond_str(
                &ctx.user_lang()
                    .autostar_filters_remove_success(asc.name, group.name),
                false,
            )
            .await?;
        } else {
            ctx.respond_str(
                &ctx.user_lang()
                    .autostar_filters_remove_fail(asc.name, group.name),
                true,
            )
            .await?;
        }

        Ok(())
    }
}
