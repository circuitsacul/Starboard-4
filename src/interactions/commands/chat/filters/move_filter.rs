use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::models::{filter::Filter, filter_group::FilterGroup},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    locale_func,
    utils::id_as_i64::GetI64,
};

locale_func!(filters_move);
locale_func!(filters_move_option_group);
locale_func!(filters_move_option_current_position);
locale_func!(filters_move_option_new_position);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "move-filter",
    desc = "Change the position of a filter.",
    desc_localizations = "filters_move"
)]
pub struct MoveFilter {
    /// The filter group containing the filter to be moved.
    #[command(autocomplete = true, desc_localizations = "filters_move_option_group")]
    group: String,

    /// The original position of the filter.
    #[command(
        rename = "current-position",
        min_value = 1,
        max_value = 1_000,
        desc_localizations = "filters_move_option_current_position"
    )]
    current_position: i64,

    /// The new position of the filter.
    #[command(
        rename = "new-position",
        min_value = 1,
        max_value = 1_000,
        desc_localizations = "filters_move_option_new_position"
    )]
    new_position: i64,
}

impl MoveFilter {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();
        let lang = ctx.user_lang();

        let group = FilterGroup::get_by_name(&ctx.bot.pool, guild_id, &self.group).await?;
        let Some(group) = group else {
            ctx.respond_str(&lang.filter_group_missing(self.group), true).await?;
            return Ok(());
        };

        let ret = Filter::set_position(
            &ctx.bot.pool,
            group.id,
            self.current_position as i16,
            self.new_position as i16,
        )
        .await?;
        if ret.is_some() {
            ctx.respond_str(
                &lang.filters_move_done(self.new_position, self.current_position),
                false,
            )
            .await?;
        } else {
            ctx.respond_str(
                &lang.filter_missing(self.group, self.current_position),
                true,
            )
            .await?;
        }

        Ok(())
    }
}
