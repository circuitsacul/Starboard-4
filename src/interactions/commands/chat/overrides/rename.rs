use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::{validation, StarboardOverride},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    locale_func,
    utils::{id_as_i64::GetI64, pg_error::PgErrorTraits},
};

locale_func!(overrides_rename);
locale_func!(overrides_rename_option_current_name);
locale_func!(overrides_rename_option_new_name);

#[derive(CreateCommand, CommandModel)]
#[command(
    name = "rename",
    desc = "Rename an override.",
    desc_localizations = "overrides_rename"
)]
pub struct RenameOverride {
    /// The current name of the override.
    #[command(
        autocomplete = true,
        rename = "current-name",
        desc_localizations = "overrides_rename_option_current_name"
    )]
    current_name: String,

    /// The new name of the override.
    #[command(
        rename = "new-name",
        desc_localizations = "overrides_rename_option_new_name"
    )]
    new_name: String,
}

impl RenameOverride {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();
        let lang = ctx.user_lang();

        let name = match validation::name::validate_name(&self.new_name) {
            Ok(val) => val,
            Err(why) => {
                ctx.respond_str(&why, true).await?;
                return Ok(());
            }
        };

        let ov =
            StarboardOverride::rename(&ctx.bot.pool, guild_id, &self.current_name, &name).await;

        match ov {
            Err(why) => {
                if why.is_duplicate() {
                    ctx.respond_str(&lang.override_already_exists(name), true)
                        .await?;
                } else {
                    return Err(why.into());
                }
            }
            Ok(None) => {
                ctx.respond_str(&lang.override_missing(self.current_name), true)
                    .await?;
            }
            Ok(Some(_)) => {
                ctx.respond_str(&lang.overrides_rename_done(self.current_name, name), false)
                    .await?;
            }
        }

        Ok(())
    }
}
