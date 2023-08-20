use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    core::premium::{is_premium::is_guild_premium, locks::refresh_premium_locks},
    database::AutoStarChannel,
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    locale_func,
    utils::{id_as_i64::GetI64, views::confirm},
};

locale_func!(autostar_delete);
locale_func!(autostar_delete_option_name);

#[derive(CreateCommand, CommandModel)]
#[command(
    name = "delete",
    desc = "Delete an autostar channel.",
    desc_localizations = "autostar_delete"
)]
pub struct DeleteAutoStarChannel {
    /// The name of the autostar channel to delete.
    #[command(
        autocomplete = true,
        desc_localizations = "autostar_delete_option_name"
    )]
    name: String,
}

impl DeleteAutoStarChannel {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);

        let conf = ctx.user_lang().autostar_delete_confirm(&self.name);
        let mut btn_ctx = match confirm::simple(&mut ctx, &conf, true).await? {
            None => return Ok(()),
            Some(btn_ctx) => btn_ctx,
        };

        let ret = AutoStarChannel::delete(&ctx.bot.pool, &self.name, guild_id.get_i64()).await?;
        refresh_premium_locks(
            &ctx.bot,
            guild_id.get_i64(),
            is_guild_premium(&ctx.bot, guild_id.get_i64(), true).await?,
        )
        .await?;
        if ret.is_none() {
            btn_ctx
                .edit_str(&ctx.user_lang().autostar_channel_missing(self.name), true)
                .await?;
        } else {
            btn_ctx
                .edit_str(&ctx.user_lang().autostar_delete_done(self.name), true)
                .await?;
        }
        Ok(())
    }
}
