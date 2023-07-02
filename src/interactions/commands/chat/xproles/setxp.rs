use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_mention::Mention;
use twilight_model::guild::Role;

use crate::{
    constants, core::premium::is_premium::is_guild_premium, database::XPRole,
    errors::StarboardResult, get_guild_id, interactions::context::CommandCtx, locale_func,
    utils::id_as_i64::GetI64,
};

locale_func!(xproles_setxp);
locale_func!(xproles_setxp_role);
locale_func!(xproles_setxp_required_xp);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "setxp",
    desc = "Create or modify an XP-based award role.",
    desc_localizations = "xproles_setxp"
)]
pub struct SetXP {
    /// The role to use as an XP-based award role.
    #[command(desc_localizations = "xproles_setxp_role")]
    role: Role,
    /// How much XP is required to obtain this award role.
    #[command(
        min_value = 1,
        max_value = 32_767,
        rename = "required-xp",
        desc_localizations = "xproles_setxp_required_xp"
    )]
    required_xp: i64,
}

impl SetXP {
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

        let count = XPRole::count(&ctx.bot.pool, guild_id).await?;
        if count >= constants::MAX_XPROLES {
            ctx.respond_str(lang.xproles_setxp_limit(constants::MAX_XPROLES), true)
                .await?;
            return Ok(());
        }

        let role_id = self.role.id.get_i64();
        let xprole =
            XPRole::create(&ctx.bot.pool, role_id, guild_id, self.required_xp as i16).await?;

        if xprole.is_none() {
            XPRole::set_required(&ctx.bot.pool, role_id, self.required_xp as i16).await?;
            ctx.respond_str(
                lang.xproles_setxp_edit(self.role.mention(), self.required_xp),
                false,
            )
            .await?;
        } else {
            ctx.respond_str(
                lang.xproles_setxp_create(self.role.mention(), self.required_xp),
                false,
            )
            .await?;
        }

        Ok(())
    }
}
