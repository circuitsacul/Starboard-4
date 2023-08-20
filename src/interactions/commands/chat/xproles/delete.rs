use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_mention::Mention;
use twilight_model::guild::Role;

use crate::{
    database::XPRole,
    errors::StarboardResult,
    get_guild_id,
    interactions::{commands::deleted_roles::get_deleted_roles, context::CommandCtx},
    locale_func,
    utils::{id_as_i64::GetI64, into_id::IntoId, views::confirm},
};

locale_func!(xproles_delete);
locale_func!(xproles_delete_option_xprole);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "delete",
    desc = "Delete an XP-based award role.",
    desc_localizations = "xproles_delete"
)]
pub struct Delete {
    /// The XPRole to delete.
    #[command(desc_localizations = "xproles_delete_option_xprole")]
    xprole: Role,
}

impl Delete {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let lang = ctx.user_lang();

        let role = XPRole::delete(&ctx.bot.pool, self.xprole.id.get_i64()).await?;

        let (msg, ephemeral) = match role {
            None => (lang.xprole_missing(self.xprole.mention()), true),
            Some(_) => (lang.xproles_delete_done(self.xprole.mention()), false),
        };
        ctx.respond_str(msg, ephemeral).await?;

        Ok(())
    }
}

locale_func!(xproles_clear_deleted);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "clear-deleted",
    desc = "Delete XPRoles if the Discord role has been deleted.",
    desc_localizations = "xproles_clear_deleted"
)]
pub struct ClearDeleted;

impl ClearDeleted {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);
        let guild_id_i64 = guild_id.get_i64();
        let lang = ctx.user_lang();

        let xpr = XPRole::list_by_guild(&ctx.bot.pool, guild_id_i64).await?;

        let (to_delete_pretty, to_delete) = get_deleted_roles(
            &ctx.bot,
            guild_id,
            xpr.into_iter().map(|r| r.role_id.into_id()),
        );

        if to_delete.is_empty() {
            ctx.respond_str(lang.xproles_clear_deleted_none(), true)
                .await?;
            return Ok(());
        }

        let conf = confirm::simple(
            &mut ctx,
            lang.xproles_clear_deleted_confirm(to_delete_pretty),
            true,
        )
        .await?;

        let Some(mut btn_ctx) = conf else {
            return Ok(());
        };

        for role in to_delete {
            XPRole::delete(&ctx.bot.pool, role.get_i64()).await?;
        }

        btn_ctx
            .edit_str(lang.xproles_clear_deleted_done(), true)
            .await?;

        Ok(())
    }
}
