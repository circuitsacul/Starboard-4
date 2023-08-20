use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    constants,
    database::models::{filter::Filter, filter_group::FilterGroup},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    locale_func,
    utils::id_as_i64::GetI64,
};

locale_func!(filters_create);
locale_func!(filters_create_option_group);
locale_func!(filters_create_option_position);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "create-filter",
    desc = "Create a filter for a filter group.",
    desc_localizations = "filters_create"
)]
pub struct CreateFilter {
    /// The filter group to create this filter for.
    #[command(
        autocomplete = true,
        desc_localizations = "filters_create_option_group"
    )]
    group: String,
    /// The position to put the filter in. Use 1 for the start (top) or leave blank for the end.
    #[command(
        min_value = 1,
        max_value = 1_000,
        desc_localizations = "filters_create_option_position"
    )]
    position: Option<i64>,
}

impl CreateFilter {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();

        let Some(group) = FilterGroup::get_by_name(&ctx.bot.pool, guild_id, &self.group).await? else {
            ctx.respond_str(&ctx.user_lang().filter_group_missing(self.group), true).await?;
            return Ok(());
        };

        let count = Filter::list_by_filter(&ctx.bot.pool, group.id).await?.len();
        if count >= constants::MAX_FILTERS_PER_GROUP {
            ctx.respond_str(
                &ctx.user_lang()
                    .filters_create_limit_reached(constants::MAX_FILTERS_PER_GROUP),
                true,
            )
            .await?;
            return Ok(());
        }

        if let Some(insert_pos) = self.position {
            Filter::shift(&ctx.bot.pool, group.id, insert_pos as i16, None, 1).await?;
        }

        let position = match self.position {
            Some(val) => val as i16,
            None => Filter::get_last_position(&ctx.bot.pool, group.id).await? + 1,
        };

        Filter::create(&ctx.bot.pool, group.id, position)
            .await?
            .unwrap();

        ctx.respond_str(
            &ctx.user_lang().filters_create_done(group.name, position),
            false,
        )
        .await?;
        Ok(())
    }
}
