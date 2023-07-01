use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::models::filter_group::FilterGroup,
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    locale_func,
    utils::{id_as_i64::GetI64, views::confirm},
};

locale_func!(filters_delete_group);
locale_func!(filters_delete_group_option_name);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "delete-group",
    desc = "Delete a filter group.",
    desc_localizations = "filters_delete_group"
)]
pub struct DeleteGroup {
    /// The filter group to delete.
    #[command(
        autocomplete = true,
        desc_localizations = "filters_delete_group_option_name"
    )]
    name: String,
}

impl DeleteGroup {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();

        let conf = ctx.user_lang().filters_delete_group_confirm(&self.name);
        let Some(mut btn_ctx) = confirm::simple(
            &mut ctx,
            &conf,
            true,
        ).await? else {
            return Ok(());
        };

        let filter = FilterGroup::delete(&ctx.bot.pool, guild_id, &self.name).await?;

        if filter.is_some() {
            btn_ctx
                .edit_str(&ctx.user_lang().filters_delete_group_done(&self.name), true)
                .await?;
        } else {
            btn_ctx
                .edit_str(&ctx.user_lang().filter_group_missing(&self.name), true)
                .await?;
        }

        Ok(())
    }
}
