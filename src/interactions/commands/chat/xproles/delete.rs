use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::guild::Role;

use crate::{
    concat_format,
    database::XPRole,
    errors::StarboardResult,
    get_guild_id,
    interactions::{commands::deleted_roles::get_deleted_roles, context::CommandCtx},
    utils::{id_as_i64::GetI64, into_id::IntoId, views::confirm},
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "delete", desc = "Delete an XP-based award role.")]
pub struct Delete {
    /// The XPRole to delete.
    xprole: Role,
}

impl Delete {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let role = XPRole::delete(&ctx.bot.pool, self.xprole.id.get_i64()).await?;

        let (msg, ephemeral) = match role {
            None => ("That is not an XPRole.", true),
            Some(_) => ("XPRole deleted.", false),
        };
        ctx.respond_str(msg, ephemeral).await?;

        Ok(())
    }
}

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "clear-deleted",
    desc = "Delete XPRoles if the Discord role has been deleted."
)]
pub struct ClearDeleted;

impl ClearDeleted {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);
        let guild_id_i64 = guild_id.get_i64();

        let xpr = XPRole::list_by_guild(&ctx.bot.pool, guild_id_i64).await?;

        let (to_delete_pretty, to_delete) = get_deleted_roles(
            &ctx.bot,
            guild_id,
            xpr.into_iter().map(|r| r.role_id.into_id()),
        );

        if to_delete.is_empty() {
            ctx.respond_str("Nothing to clear.", true).await?;
            return Ok(());
        }

        let conf = confirm::simple(
            &mut ctx,
            &concat_format!(
                "This will delete the following XPRoles:\n";
                "{to_delete_pretty}\n";
                "Do you wish to continue?";
            ),
            true,
        )
        .await?;

        let Some(mut btn_ctx) = conf else {
            return Ok(());
        };

        for role in to_delete {
            XPRole::delete(&ctx.bot.pool, role.get_i64()).await?;
        }

        btn_ctx.edit_str("Done.", true).await?;

        Ok(())
    }
}
