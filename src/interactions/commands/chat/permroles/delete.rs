use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_mention::Mention;
use twilight_model::guild::Role;

use crate::{
    database::PermRole,
    errors::StarboardResult,
    get_guild_id,
    interactions::{commands::deleted_roles::get_deleted_roles, context::CommandCtx},
    locale_func,
    utils::{id_as_i64::GetI64, into_id::IntoId, views::confirm},
};

locale_func!(permroles_delete);
locale_func!(permroles_delete_option_role);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "delete",
    desc = "Delete a PermRole.",
    desc_localizations = "permroles_delete"
)]
pub struct DeletePermRole {
    /// The PermRole to delete.
    #[command(desc_localizations = "permroles_delete_option_role")]
    role: Role,
}

impl DeletePermRole {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let lang = ctx.user_lang();
        let pr = PermRole::delete(&ctx.bot.pool, self.role.id.get_i64()).await?;
        if pr.is_none() {
            ctx.respond_str(&lang.permrole_missing(self.role.mention()), true)
                .await?;
        } else {
            ctx.respond_str(&lang.permroles_delete_done(self.role.mention()), false)
                .await?;
        }

        Ok(())
    }
}

locale_func!(permroles_clear_deleted);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "clear-deleted",
    desc = "Delete PermRoles if the Discord role has been deleted.",
    desc_localizations = "permroles_clear_deleted"
)]
pub struct ClearDeleted;

impl ClearDeleted {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);
        let guild_id_i64 = guild_id.get_i64();
        let lang = ctx.user_lang();

        let pr = PermRole::list_by_guild(&ctx.bot.pool, guild_id_i64).await?;

        let (to_delete_pretty, to_delete) = get_deleted_roles(
            &ctx.bot,
            guild_id,
            pr.into_iter().map(|r| r.role_id.into_id()),
        );

        if to_delete.is_empty() {
            ctx.respond_str(lang.permroles_clear_deleted_nothing(), true)
                .await?;
            return Ok(());
        }

        let conf = confirm::simple(
            &mut ctx,
            lang.permroles_clear_deleted_confirm(to_delete_pretty),
            true,
        )
        .await?;

        let Some(mut btn_ctx) = conf else {
            return Ok(());
        };

        for role in to_delete {
            PermRole::delete(&ctx.bot.pool, role.get_i64()).await?;
        }

        btn_ctx
            .edit_str(lang.permroles_clear_deleted_done(), true)
            .await?;

        Ok(())
    }
}
