use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_mention::Mention;
use twilight_model::guild::Role;

use crate::{
    database::PermRole, errors::StarboardResult, interactions::context::CommandCtx, unwrap_id,
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "delete", desc = "Delete a PermRole.")]
pub struct DeletePermRole {
    /// The PermRole to delete.
    role: Role,
}

impl DeletePermRole {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let pr = PermRole::delete(&ctx.bot.pool, unwrap_id!(self.role.id)).await?;
        if pr.is_none() {
            ctx.respond_str(&format!("{} is not a PermRole.", self.role.mention()), true)
                .await?;
        } else {
            ctx.respond_str(&format!("Deleted PermRole {}.", self.role.mention()), false)
                .await?;
        }

        Ok(())
    }
}
