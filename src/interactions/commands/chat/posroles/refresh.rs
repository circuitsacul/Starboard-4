use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    core::{posroles::update_posroles_for_guild, premium::is_premium::is_guild_premium},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    locale_func,
    utils::id_as_i64::GetI64,
};

locale_func!(posroles_refresh);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "refresh",
    desc = "Refresh the PosRoles for the server.",
    desc_localizations = "posroles_refresh"
)]
pub struct Refresh;

impl Refresh {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);
        let lang = ctx.user_lang();

        if !is_guild_premium(&ctx.bot, guild_id.get_i64(), true).await? {
            ctx.respond_str(lang.premium_command(), true).await?;
            return Ok(());
        }

        ctx.defer(true).await?;

        let ret = update_posroles_for_guild(ctx.bot.clone(), guild_id).await?;

        if let Some(ret) = ret {
            ctx.respond_str(
                lang.posroles_refresh_done(
                    ret.added_roles,
                    ret.failed_adds,
                    ret.removed_roles,
                    ret.failed_removals,
                ),
                true,
            )
            .await?;
        } else {
            ctx.respond_str(lang.posroles_refresh_already_refreshing(), true)
                .await?;
        }

        Ok(())
    }
}
