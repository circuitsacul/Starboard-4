use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_mention::Mention;
use twilight_model::guild::Role;

use crate::{
    database::PosRole,
    errors::StarboardResult,
    get_guild_id,
    interactions::{commands::deleted_roles::get_deleted_roles, context::CommandCtx},
    locale_func,
    utils::{id_as_i64::GetI64, into_id::IntoId, views::confirm},
};

locale_func!(posroles_delete);
locale_func!(posroles_delete_option_posrole);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "delete",
    desc = "Delete a position-based award role.",
    desc_localizations = "posroles_delete"
)]
pub struct Delete {
    /// The PosRole to delete.
    #[command(desc_localizations = "posroles_delete_option_posrole")]
    posrole: Role,
}

impl Delete {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let lang = ctx.user_lang();

        let role = PosRole::delete(&ctx.bot.pool, self.posrole.id.get_i64()).await?;

        let (msg, ephemeral) = match role {
            None => (lang.posrole_missing(self.posrole.mention()), true),
            Some(_) => (lang.posroles_delete_done(self.posrole.mention()), false),
        };
        ctx.respond_str(msg, ephemeral).await?;

        Ok(())
    }
}

locale_func!(posroles_clear_deleted);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "clear-deleted",
    desc = "Delete PosRoles if the Discord role has been deleted.",
    desc_localizations = "posroles_clear_deleted"
)]
pub struct ClearDeleted;

impl ClearDeleted {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);
        let guild_id_i64 = guild_id.get_i64();
        let lang = ctx.user_lang();

        let pr = PosRole::list_by_guild(&ctx.bot.pool, guild_id_i64).await?;

        let (to_delete_pretty, to_delete) = get_deleted_roles(
            &ctx.bot,
            guild_id,
            pr.into_iter().map(|r| r.role_id.into_id()),
        );

        if to_delete.is_empty() {
            ctx.respond_str(lang.posroles_clear_deleted_nothing(), true)
                .await?;
            return Ok(());
        }

        let conf = confirm::simple(
            &mut ctx,
            lang.posroles_clear_deleted_confirm(to_delete_pretty),
            true,
        )
        .await?;

        let Some(mut btn_ctx) = conf else {
            return Ok(());
        };

        for role in to_delete {
            PosRole::delete(&ctx.bot.pool, role.get_i64()).await?;
        }

        btn_ctx
            .edit_str(lang.posroles_clear_deleted_done(), true)
            .await?;

        Ok(())
    }
}
