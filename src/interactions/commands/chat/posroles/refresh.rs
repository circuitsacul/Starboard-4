use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    concat_format, core::posroles::update_posroles_for_guild, errors::StarboardResult,
    get_guild_id, interactions::context::CommandCtx,
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "refresh", desc = "Refresh the PosRoles for the server.")]
pub struct Refresh;

impl Refresh {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);

        ctx.defer(true).await?;

        let ret = update_posroles_for_guild(ctx.bot.clone(), guild_id).await?;

        ctx.respond_str(
            &concat_format!(
                "Finished updating.\n";
                "Added {} roles, {} failed.\n" <- ret.added_roles, ret.failed_adds;
                "Removed {} roles, {} failed." <- ret.removed_roles, ret.failed_removals;
            ),
            true,
        )
        .await?;

        Ok(())
    }
}
