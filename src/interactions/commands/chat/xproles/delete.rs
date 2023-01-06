use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::guild::Role;

use crate::{
    database::XPRole, errors::StarboardResult, interactions::context::CommandCtx,
    utils::id_as_i64::GetI64,
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
