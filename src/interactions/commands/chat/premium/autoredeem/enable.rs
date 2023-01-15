use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::Member, errors::StarboardResult, interactions::context::CommandCtx, map_dup_none,
    utils::id_as_i64::GetI64,
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "enable", desc = "Enable autoredeem for the current server.")]
pub struct Enable;

impl Enable {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let Some(guild_id) = ctx.interaction.guild_id else {
            ctx.respond_str(
                "Please run this command inside a server.",
                true
            ).await?;
            return Ok(());
        };
        let guild_id = guild_id.get_i64();
        let user_id = ctx.interaction.author_id().unwrap().get_i64();

        map_dup_none!(Member::create(&ctx.bot.pool, user_id, guild_id))?;

        Member::set_autoredeem_enabled(&ctx.bot.pool, user_id, guild_id, true).await?;

        ctx.respond_str("Autoredeem enabled.", true).await?;

        Ok(())
    }
}
