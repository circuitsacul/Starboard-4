use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::DbMember, errors::StarboardResult, interactions::context::CommandCtx, locale_func,
    utils::id_as_i64::GetI64,
};

locale_func!(autoredeem_disable);
locale_func!(autoredeem_disable_option_server);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "disable",
    desc = "Disable autoredeem for a server.",
    desc_localizations = "autoredeem_disable"
)]
pub struct Disable {
    /// The server to disable autoredeem for.
    #[command(
        autocomplete = true,
        desc_localizations = "autoredeem_disable_option_server"
    )]
    server: Option<String>,
}

impl Disable {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let user_id = ctx.interaction.author_id().unwrap().get_i64();
        let lang = ctx.user_lang();

        let guild_id = 'out: {
            let Some(input_guild) = self.server else {
                let Some(guild_id) = ctx.interaction.guild_id else {
                    ctx.respond_str(
                        lang.autoredeem_disable_invalid_guild(),
                        true
                    ).await?;
                    return Ok(());
                };

                break 'out guild_id.get_i64();
            };

            let Ok(guild_id) = input_guild.parse::<i64>() else {
                ctx.respond_str(
                    lang.autoredeem_disable_invalid_guild(),
                    true
                ).await?;
                return Ok(());
            };

            guild_id
        };

        DbMember::set_autoredeem_enabled(&ctx.bot.pool, user_id, guild_id, false).await?;

        ctx.respond_str(lang.autoredeem_disable_done(), true)
            .await?;

        Ok(())
    }
}
