use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_mention::Mention;
use twilight_model::guild::Role;

use crate::{
    database::PermRole, get_guild_id, interactions::context::CommandCtx, map_dup_none, unwrap_id,
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "create", desc = "Create a PermRole.")]
pub struct CreatePermRole {
    /// The role to use as a PermRole.
    role: Role,
}

impl CreatePermRole {
    pub async fn callback(self, mut ctx: CommandCtx) -> anyhow::Result<()> {
        let guild_id = get_guild_id!(ctx);

        let pr = map_dup_none!(PermRole::create(
            &ctx.bot.pool,
            unwrap_id!(self.role.id),
            unwrap_id!(guild_id),
        ))?;

        if pr.is_none() {
            ctx.respond_str("That is already a PermRole.", true).await?;
        } else {
            ctx.respond_str(
                &format!("{} is now a PermRole.", self.role.mention()),
                false,
            )
            .await?;
        }

        Ok(())
    }
}
