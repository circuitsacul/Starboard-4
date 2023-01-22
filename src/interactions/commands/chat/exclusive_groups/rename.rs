use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::{validation::name::validate_name, ExclusiveGroup},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    map_dup_none,
    utils::id_as_i64::GetI64,
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "rename", desc = "Rename an exclusive group.")]
pub struct Rename {
    /// The original name for the exclusive group.
    #[command(rename = "original-name", autocomplete = true)]
    original_name: String,
    /// The new name for the exclusive group.
    #[command(rename = "new-name")]
    new_name: String,
}

impl Rename {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();

        let new_name = match validate_name(&self.new_name) {
            Err(why) => {
                ctx.respond_str(&why, true).await?;
                return Ok(());
            }
            Ok(name) => name,
        };

        let ret = map_dup_none!(ExclusiveGroup::rename(
            &ctx.bot.pool,
            guild_id,
            &self.original_name,
            &new_name,
        ))?;

        let err = match ret {
            None => format!("An exclusive group named '{new_name}' already exists."),
            Some(None) => format!("Exclusive group '{}' does not exist.", self.original_name),
            Some(Some(_)) => {
                ctx.respond_str("Done.", true).await?;
                return Ok(());
            }
        };
        ctx.respond_str(&err, true).await?;

        Ok(())
    }
}
