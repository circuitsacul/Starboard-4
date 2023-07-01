use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::ExclusiveGroup, errors::StarboardResult, get_guild_id,
    interactions::context::CommandCtx, locale_func, utils::id_as_i64::GetI64,
};

locale_func!(exclusive_groups_delete);
locale_func!(exclusive_groups_delete_option_name);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "delete",
    desc = "Delete an exclusive group.",
    desc_localizations = "exclusive_groups_delete"
)]
pub struct Delete {
    /// The exclusive group to delete.
    #[command(
        autocomplete = true,
        desc_localizations = "exclusive_groups_delete_option_name"
    )]
    name: String,
}

impl Delete {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();

        let ret = ExclusiveGroup::delete(&ctx.bot.pool, &self.name, guild_id).await?;
        let Some(group) = ret else {
            ctx.respond_str(
                &ctx.user_lang().exclusive_group_missing(self.name),
                true,
            )
            .await?;
            return Ok(());
        };

        sqlx::query!(
            "UPDATE overrides SET overrides = (overrides::jsonb - 'exclusive_group')::json
            WHERE guild_id=$1 AND (overrides::jsonb->'exclusive_group')::int=$2",
            guild_id,
            group.id,
        )
        .fetch_all(&ctx.bot.pool)
        .await?;

        ctx.respond_str(
            &ctx.user_lang().exclusive_groups_delete_done(self.name),
            false,
        )
        .await?;

        Ok(())
    }
}
