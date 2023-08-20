use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    constants,
    database::{validation::name::validate_name, DbGuild, ExclusiveGroup},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    locale_func,
    utils::id_as_i64::GetI64,
};

locale_func!(exclusive_groups_create);
locale_func!(exclusive_groups_create_option_name);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "create",
    desc = "Create an exclusive group for starboards.",
    desc_localizations = "exclusive_groups_create"
)]
pub struct Create {
    /// The name for the exclusive group.
    #[command(desc_localizations = "exclusive_groups_create_option_name")]
    name: String,
}

impl Create {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();

        DbGuild::create(&ctx.bot.pool, guild_id).await?;

        let name = match validate_name(&self.name) {
            Err(why) => {
                ctx.respond_str(&why, true).await?;
                return Ok(());
            }
            Ok(name) => name,
        };

        let count = ExclusiveGroup::count_by_guild(&ctx.bot.pool, guild_id).await?;
        if count >= constants::MAX_EXCLUSIVE_GROUPS {
            ctx.respond_str(
                &ctx.user_lang()
                    .exclusive_groups_create_limit_reached(constants::MAX_EXCLUSIVE_GROUPS),
                true,
            )
            .await?;
            return Ok(());
        }

        let group = ExclusiveGroup::create(&ctx.bot.pool, &name, guild_id).await?;

        if group.is_some() {
            ctx.respond_str(&ctx.user_lang().exclusive_groups_create_done(name), false)
                .await?;
        } else {
            ctx.respond_str(&ctx.user_lang().exclusive_group_already_exists(name), true)
                .await?;
        }

        Ok(())
    }
}
