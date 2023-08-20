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

locale_func!(autostar_filters_add);
locale_func!(autostar_filters_add_option_autostar_channel);
locale_func!(autostar_filters_add_option_filter_group);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "add",
    desc = "Add a filter group to an autostar channel.",
    desc_localizations = "autostar_filters_add"
)]
pub struct Add {
    /// The autostar channel to add the filter to.
    #[command(
        autocomplete = true,
        rename = "autostar-channel",
        desc_localizations = "autostar_filters_add_option_autostar_channel"
    )]
    autostar_channel: String,
    /// The filter group to add to the autostar channel.
    #[command(
        autocomplete = true,
        rename = "filter-group",
        desc_localizations = "autostar_filters_add_option_filter_group"
    )]
    filter_group: String,
}

impl Add {
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

        let ret = AutostarChannelFilterGroup::create(&ctx.bot.pool, group.id, asc.id).await?;
        if ret.is_some() {
            ctx.respond_str(
                &ctx.user_lang()
                    .autostar_filters_add_success(self.autostar_channel, self.filter_group),
                false,
            )
            .await?;
        } else {
            ctx.respond_str(
                &ctx.user_lang()
                    .autostar_filters_add_already_added(self.autostar_channel, self.filter_group),
                true,
            )
            .await?;
        }

        Ok(())
    }
}
