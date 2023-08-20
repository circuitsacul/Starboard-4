use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::DbMember, errors::StarboardResult, interactions::context::CommandCtx, locale_func,
    utils::id_as_i64::GetI64,
};

locale_func!(autoredeem_enable);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "enable",
    desc = "Enable autoredeem for the current server.",
    desc_localizations = "autoredeem_enable"
)]
pub struct Enable;

impl Enable {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let lang = ctx.user_lang();

        let Some(guild_id) = ctx.interaction.guild_id else {
            ctx.respond_str(
                lang.autoredeem_enable_dm(),
                true
            ).await?;
            return Ok(());
        };
        let guild_id = guild_id.get_i64();
        let user_id = ctx.interaction.author_id().unwrap().get_i64();

        DbMember::create(&ctx.bot.pool, user_id, guild_id).await?;
        DbMember::set_autoredeem_enabled(&ctx.bot.pool, user_id, guild_id, true).await?;

        ctx.respond_str(lang.autoredeem_enable_done(), true).await?;

        Ok(())
    }
}
