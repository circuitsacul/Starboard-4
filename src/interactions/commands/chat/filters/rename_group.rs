use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::models::filter_group::FilterGroup,
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    locale_func,
    utils::{id_as_i64::GetI64, pg_error::PgErrorTraits},
};

locale_func!(filters_rename_group);
locale_func!(filters_rename_group_option_current_name);
locale_func!(filters_rename_group_option_new_name);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "rename-group",
    desc = "Rename a filter group.",
    desc_localizations = "filters_rename_group"
)]
pub struct RenameGroup {
    /// The current name of the group.
    #[command(
        autocomplete = true,
        rename = "current-name",
        desc_localizations = "filters_rename_group_option_current_name"
    )]
    current_name: String,

    /// The new name of the group.
    #[command(
        rename = "new-name",
        desc_localizations = "filters_rename_group_option_new_name"
    )]
    new_name: String,
}

impl RenameGroup {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();
        let lang = ctx.user_lang();

        let group = FilterGroup::get_by_name(&ctx.bot.pool, guild_id, &self.current_name).await?;
        let Some(group) = group else {
            ctx.respond_str(&lang.filter_group_missing(self.current_name), true).await?;
            return Ok(());
        };

        let ret = FilterGroup::rename(&ctx.bot.pool, group.id, &self.new_name).await;

        match ret {
            Ok(_) => {
                ctx.respond_str(
                    &lang.filters_rename_group_done(self.new_name, self.current_name),
                    false,
                )
                .await?;
            }
            Err(why) => {
                if why.is_duplicate() {
                    ctx.respond_str(&lang.filter_group_already_exists(self.new_name), true)
                        .await?;
                } else {
                    return Err(why.into());
                }
            }
        }

        Ok(())
    }
}
