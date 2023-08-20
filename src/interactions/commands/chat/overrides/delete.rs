use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::StarboardOverride,
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    locale_func,
    utils::{id_as_i64::GetI64, views::confirm},
};

locale_func!(overrides_delete);
locale_func!(overrides_delete_option_name);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "delete",
    desc = "Delete an override.",
    desc_localizations = "overrides_delete"
)]
pub struct DeleteOverride {
    /// The name of the override to delete.
    #[command(
        autocomplete = true,
        desc_localizations = "overrides_delete_option_name"
    )]
    name: String,
}

impl DeleteOverride {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();
        let lang = ctx.user_lang();

        let btn_ctx =
            confirm::simple(&mut ctx, &lang.overrides_delete_confirm(&self.name), true).await?;
        let mut btn_ctx = match btn_ctx {
            None => return Ok(()),
            Some(btn_ctx) => btn_ctx,
        };

        let ov = StarboardOverride::delete(&ctx.bot.pool, guild_id, &self.name).await?;
        if ov.is_none() {
            btn_ctx
                .edit_str(&lang.override_missing(self.name), true)
                .await?;
        } else {
            btn_ctx
                .edit_str(&lang.overrides_delete_done(self.name), true)
                .await?;
        }

        Ok(())
    }
}
