use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::models::filter::FilterGroup, errors::StarboardResult, get_guild_id,
    interactions::context::CommandCtx, utils::id_as_i64::GetI64,
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "delete-group", desc = "Delete a filter group.")]
pub struct DeleteGroup {
    /// The filter group to delete.
    #[command(autocomplete = true)]
    name: String,
}

impl DeleteGroup {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();

        let filter = FilterGroup::delete(&ctx.bot.pool, guild_id, &self.name).await?;

        if filter.is_some() {
            ctx.respond_str(&format!("Deleted filter group '{}'.", self.name), false)
                .await?;
        } else {
            ctx.respond_str(
                &format!("Filter group '{}' does not exist.", self.name),
                true,
            )
            .await?;
        }

        Ok(())
    }
}
