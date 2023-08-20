use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    constants,
    database::{models::filter_group::FilterGroup, validation::name::validate_name, DbGuild},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    locale_func,
    utils::id_as_i64::GetI64,
};

locale_func!(filters_create_group);
locale_func!(filters_create_group_option_name);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "create-group",
    desc = "Create a filter group.",
    desc_localizations = "filters_create_group"
)]
pub struct CreateGroup {
    /// The name of the filter group.
    #[command(desc_localizations = "filters_create_group_option_name")]
    name: String,
}

impl CreateGroup {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();

        let count = FilterGroup::list_by_guild(&ctx.bot.pool, guild_id)
            .await?
            .len();
        if count >= constants::MAX_FILTER_GROUPS {
            ctx.respond_str(
                &ctx.user_lang()
                    .filters_create_group_limit_reached(constants::MAX_FILTER_GROUPS),
                true,
            )
            .await?;
            return Ok(());
        }

        DbGuild::create(&ctx.bot.pool, guild_id).await?;
        let name = match validate_name(&self.name) {
            Ok(val) => val,
            Err(why) => {
                ctx.respond_str(&why, true).await?;
                return Ok(());
            }
        };
        let group = FilterGroup::create(&ctx.bot.pool, guild_id, &name).await?;
        if group.is_none() {
            ctx.respond_str(&ctx.user_lang().filter_group_already_exists(name), true)
                .await?;
        } else {
            ctx.respond_str(&ctx.user_lang().filters_create_group_done(name), false)
                .await?;
        }

        Ok(())
    }
}
