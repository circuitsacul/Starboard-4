use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::{validation::name::validate_name, ExclusiveGroup},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    locale_func,
    utils::{id_as_i64::GetI64, pg_error::PgErrorTraits},
};

locale_func!(exclusive_groups_rename);
locale_func!(exclusive_groups_rename_option_original_name);
locale_func!(exclusive_groups_rename_option_new_name);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "rename",
    desc = "Rename an exclusive group.",
    desc_localizations = "exclusive_groups_rename"
)]
pub struct Rename {
    /// The original name for the exclusive group.
    #[command(
        rename = "original-name",
        autocomplete = true,
        desc_localizations = "exclusive_groups_rename_option_original_name"
    )]
    original_name: String,
    /// The new name for the exclusive group.
    #[command(
        rename = "new-name",
        desc_localizations = "exclusive_groups_rename_option_new_name"
    )]
    new_name: String,
}

impl Rename {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();

        let new_name = match validate_name(&self.new_name) {
            Err(why) => {
                ctx.respond_str(&why, true).await?;
                return Ok(());
            }
            Ok(name) => name,
        };

        let ret =
            ExclusiveGroup::rename(&ctx.bot.pool, guild_id, &self.original_name, &new_name).await;

        let err = match ret {
            Err(why) => {
                if why.is_duplicate() {
                    ctx.user_lang().exclusive_group_already_exists(new_name)
                } else {
                    return Err(why.into());
                }
            }
            Ok(None) => ctx.user_lang().exclusive_group_missing(self.original_name),
            Ok(Some(_)) => {
                ctx.respond_str(
                    &ctx.user_lang()
                        .exclusive_groups_rename_done(new_name, self.original_name),
                    true,
                )
                .await?;
                return Ok(());
            }
        };
        ctx.respond_str(&err, true).await?;

        Ok(())
    }
}
