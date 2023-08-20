use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_mention::Mention;
use twilight_model::guild::Role;

use crate::{
    constants, database::PermRole, errors::StarboardResult, get_guild_id,
    interactions::context::CommandCtx, locale_func, utils::id_as_i64::GetI64,
};

locale_func!(permroles_create);
locale_func!(permroles_create_option_role);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "create",
    desc = "Create a PermRole.",
    desc_localizations = "permroles_create"
)]
pub struct CreatePermRole {
    /// The role to use as a PermRole.
    #[command(desc_localizations = "permroles_create_option_role")]
    role: Role,
}

impl CreatePermRole {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);
        let guild_id_i64 = guild_id.get_i64();
        let lang = ctx.user_lang();

        let count = PermRole::count_by_guild(&ctx.bot.pool, guild_id_i64).await?;
        if count >= constants::MAX_PERMROLES {
            ctx.respond_str(&lang.permroles_create_limit(constants::MAX_PERMROLES), true)
                .await?;
            return Ok(());
        }

        let pr = PermRole::create(&ctx.bot.pool, self.role.id.get_i64(), guild_id_i64).await?;

        if pr.is_none() {
            ctx.respond_str(
                &lang.permroles_create_already_exists(self.role.mention()),
                true,
            )
            .await?;
        } else {
            ctx.respond_str(&lang.permroles_create_done(self.role.mention()), false)
                .await?;
        }

        Ok(())
    }
}
