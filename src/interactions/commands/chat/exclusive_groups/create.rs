use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    constants,
    database::{validation::name::validate_name, ExclusiveGroup, Guild},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    map_dup_none,
    utils::id_as_i64::GetI64,
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "create", desc = "Create an exclusive group for starboards.")]
pub struct Create {
    /// The name for the exclusive group.
    name: String,
}

impl Create {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();

        map_dup_none!(Guild::create(&ctx.bot.pool, guild_id))?;

        let name = match validate_name(&self.name) {
            Err(why) => {
                ctx.respond_str(&why, true).await?;
                return Ok(());
            }
            Ok(name) => name,
        };

        let count = ExclusiveGroup::count_by_guild(&ctx.bot.pool, guild_id).await?;
        if count >= constants::MAX_EXCLUSIVE_GROUPS {
            ctx.respond_str(
                &format!(
                    "You can only have up to {} exclusive groups.",
                    constants::MAX_EXCLUSIVE_GROUPS
                ),
                true,
            )
            .await?;
            return Ok(());
        }

        let group = map_dup_none!(ExclusiveGroup::create(&ctx.bot.pool, &name, guild_id))?;

        if group.is_some() {
            ctx.respond_str(&format!("Created exclusive group '{name}'."), false)
                .await?;
        } else {
            ctx.respond_str(
                &format!("An exclusive group named '{name}' already exists."),
                true,
            )
            .await?;
        }

        Ok(())
    }
}
