use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::models::filter::FilterGroup, errors::StarboardResult, get_guild_id,
    interactions::context::CommandCtx, utils::id_as_i64::GetI64,
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "create-group", desc = "Create a filter group.")]
pub struct CreateGroup {
    /// The name of the filter group.
    name: String,
}

impl CreateGroup {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();

        let group = FilterGroup::create(&ctx.bot.pool, guild_id, &self.name).await?;
        if group.is_none() {
            ctx.respond_str(
                &format!("A filter group named '{}' already exists.", self.name),
                true,
            )
            .await?;
        } else {
            ctx.respond_str(&format!("Created filter group '{}'.", self.name), true)
                .await?;
        }

        Ok(())
    }
}
