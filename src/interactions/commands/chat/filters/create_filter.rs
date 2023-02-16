use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::models::{filter::Filter, filter_group::FilterGroup},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    utils::id_as_i64::GetI64,
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "create-filter", desc = "Create a filter for a filter group.")]
pub struct CreateFilter {
    /// The filter group to create this filter for.
    #[command(autocomplete = true)]
    group: String,
    /// The position to put the filter in. Use 1 for the start (top) or leave blank for the end.
    #[command(min_value = 1, max_value = 1_000)]
    position: Option<i64>,
}

impl CreateFilter {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();

        let Some(group) = FilterGroup::get_by_name(&ctx.bot.pool, guild_id, &self.group).await? else {
            ctx.respond_str(&format!("Filter group '{}' does not exist.", self.group), true).await?;
            return Ok(());
        };

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

        ctx.respond_str("Filter created.", false).await?;
        Ok(())
    }
}
