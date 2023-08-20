use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_mention::Mention;
use twilight_model::guild::Role;

use crate::{
    constants, core::premium::is_premium::is_guild_premium, database::PosRole,
    errors::StarboardResult, get_guild_id, interactions::context::CommandCtx, locale_func,
    utils::id_as_i64::GetI64,
};

locale_func!(posroles_edit);
locale_func!(posroles_edit_option_role);
locale_func!(posroles_edit_option_max_members);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "set-max-members",
    desc = "Create or modify a position-based award role.",
    desc_localizations = "posroles_edit"
)]
pub struct SetMaxMembers {
    /// The role to use as a position-based award role.
    #[command(desc_localizations = "posroles_edit_option_role")]
    role: Role,
    /// How many members can have this award role.
    #[command(
        min_value = 1,
        rename = "max-members",
        desc_localizations = "posroles_edit_option_max_members"
    )]
    max_members: i64,
}

impl SetMaxMembers {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();
        let lang = ctx.user_lang();

        if !is_guild_premium(&ctx.bot, guild_id, true).await? {
            ctx.respond_str(lang.premium_command(), true).await?;
            return Ok(());
        }

        if self.role.id.get_i64() == guild_id || self.role.managed {
            ctx.respond_str(lang.award_role_managed(), true).await?;
            return Ok(());
        }

        let count = PosRole::count(&ctx.bot.pool, guild_id).await?;
        if count >= constants::MAX_POSROLES {
            ctx.respond_str(lang.posroles_edit_limit(constants::MAX_POSROLES), true)
                .await?;
            return Ok(());
        }

        let role_id = self.role.id.get_i64();
        let posrole =
            PosRole::create(&ctx.bot.pool, role_id, guild_id, self.max_members as i32).await?;

        if posrole.is_none() {
            PosRole::set_max_members(&ctx.bot.pool, role_id, self.max_members as i32).await?;
            ctx.respond_str(
                lang.posroles_edit_edited(self.role.mention(), self.max_members),
                false,
            )
            .await?;
        } else {
            ctx.respond_str(
                lang.posroles_edit_created(self.max_members, self.role.mention()),
                false,
            )
            .await?;
        }

        Ok(())
    }
}
