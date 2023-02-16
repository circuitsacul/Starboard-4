use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::models::{filter::Filter, filter_group::FilterGroup},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    utils::id_as_i64::GetI64,
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "edit", desc = "Edit a filters conditions.")]
pub struct Edit {
    /// The name of the filter group containing the filter to be edited.
    #[command(autocomplete = true)]
    group: String,
    /// The position of the filter to edit.
    #[command(min_value = 1, max_value = 1_000)]
    position: i64,
}

impl Edit {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();

        let Some(group) = FilterGroup::get_by_name(&ctx.bot.pool, guild_id, &self.group).await? else {
            ctx.respond_str(&format!("No filter group named '{}' exists.", self.group), true).await?;
            return Ok(());
        };

        let Some(filter) = Filter::get_by_position(&ctx.bot.pool, group.id, self.position as i16).await? else {
            ctx.respond_str(&format!("No filter for group '{}' at {} exists.", self.group, self.position), true).await?;
            return Ok(());
        };

        ctx.respond_str(&format!("{}{}", group.name, filter.position), true)
            .await?;

        Ok(())
    }
}
