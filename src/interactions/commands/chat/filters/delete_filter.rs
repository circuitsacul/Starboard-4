use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::models::{filter::Filter, filter_group::FilterGroup},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    locale_func,
    utils::{id_as_i64::GetI64, views::confirm},
};

locale_func!(filters_delete);
locale_func!(filters_delete_option_group);
locale_func!(filters_delete_option_position);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "delete-filter",
    desc = "Delete a filter from a filter group.",
    desc_localizations = "filters_delete"
)]
pub struct DeleteFilter {
    /// The group to delete the filter from.
    #[command(
        autocomplete = true,
        desc_localizations = "filters_delete_option_group"
    )]
    group: String,
    /// The position of the filter to delete.
    #[command(
        min_value = 1,
        max_value = 1_000,
        desc_localizations = "filters_delete_option_position"
    )]
    position: i64,
}

impl DeleteFilter {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();

        let group = FilterGroup::get_by_name(&ctx.bot.pool, guild_id, &self.group).await?;
        let Some(group) = group else {
            ctx.respond_str(&ctx.user_lang().filter_group_missing(self.group), true).await?;
            return Ok(());
        };

        let conf = ctx
            .user_lang()
            .filters_delete_confirm(&self.group, self.position);
        let Some(mut btn_ctx) = confirm::simple(
            &mut ctx,
            &conf,
            true,
        )
        .await? else {
            return Ok(());
        };

        let ret = Filter::delete(&ctx.bot.pool, group.id, self.position as i16).await?;

        if ret.is_some() {
            btn_ctx
                .edit_str(
                    &ctx.user_lang()
                        .filters_delete_done(self.group, self.position),
                    true,
                )
                .await?;
        } else {
            btn_ctx
                .edit_str(
                    &ctx.user_lang().filter_missing(self.group, self.position),
                    true,
                )
                .await?;
        }

        Ok(())
    }
}
