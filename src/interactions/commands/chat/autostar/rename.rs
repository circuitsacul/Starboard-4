use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::{validation, AutoStarChannel},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    locale_func,
    utils::{id_as_i64::GetI64, pg_error::PgErrorTraits},
};

locale_func!(autostar_rename);
locale_func!(autostar_rename_option_current_name);
locale_func!(autostar_rename_option_new_name);

#[derive(CreateCommand, CommandModel)]
#[command(
    name = "rename",
    desc = "Rename an autostar channel.",
    desc_localizations = "autostar_rename"
)]
pub struct RenameAutoStarChannel {
    /// The current name of the autostar channel.
    #[command(
        rename = "current-name",
        autocomplete = true,
        desc_localizations = "autostar_rename_option_current_name"
    )]
    current_name: String,

    /// The new name for the autostar channel.
    #[command(
        rename = "new-name",
        desc_localizations = "autostar_rename_option_new_name"
    )]
    new_name: String,
}

impl RenameAutoStarChannel {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);

        let new_name = match validation::name::validate_name(&self.new_name) {
            Err(why) => {
                ctx.respond_str(&why, true).await?;
                return Ok(());
            }
            Ok(name) => name,
        };

        let ret = AutoStarChannel::rename(
            &ctx.bot.pool,
            &self.current_name,
            guild_id.get_i64(),
            &new_name,
        )
        .await;

        match ret {
            Err(why) => {
                if why.is_duplicate() {
                    ctx.respond_str(
                        &ctx.user_lang().autostar_channel_already_exists(new_name),
                        true,
                    )
                    .await?
                } else {
                    return Err(why.into());
                }
            }
            Ok(None) => {
                ctx.respond_str(
                    &ctx.user_lang().autostar_channel_missing(self.current_name),
                    true,
                )
                .await?
            }
            Ok(Some(_)) => {
                ctx.respond_str(
                    &ctx.user_lang()
                        .autostar_rename_done(new_name, self.current_name),
                    false,
                )
                .await?
            }
        };

        Ok(())
    }
}
