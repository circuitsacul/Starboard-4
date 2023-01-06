use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::guild::Role;

use crate::{
    database::PosRole, errors::StarboardResult, interactions::context::CommandCtx,
    utils::id_as_i64::GetI64,
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "delete", desc = "Delete a position-based award role.")]
pub struct Delete {
    /// The PosRole to delete.
    posrole: Role,
}

impl Delete {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let role = PosRole::delete(&ctx.bot.pool, self.posrole.id.get_i64()).await?;

        let (msg, ephemeral) = match role {
            None => ("That is not a PosRole.", true),
            Some(_) => ("PosRole deleted.", false),
        };
        ctx.respond_str(msg, ephemeral).await?;

        Ok(())
    }
}
