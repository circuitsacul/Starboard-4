use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    constants,
    core::premium::redeem::{redeem_premium, RedeemPremiumResult},
    database::DbGuild,
    errors::StarboardResult,
    interactions::context::CommandCtx,
    locale_func,
    utils::{id_as_i64::GetI64, views::confirm},
};

locale_func!(premium_redeem);
locale_func!(premium_redeem_option_months);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "redeem",
    desc = "Redeem your premium credits.",
    desc_localizations = "premium_redeem"
)]
pub struct Redeem {
    /// The number of months of premium to redeem. Each month is three credits.
    #[command(
        min_value = 1,
        max_value = 6,
        desc_localizations = "premium_redeem_option_months"
    )]
    months: i64,
}

impl Redeem {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let lang = ctx.user_lang();

        let Some(guild_id) = ctx.interaction.guild_id else {
            ctx.respond_str(lang.premium_redeem_dm(), true).await?;
            return Ok(());
        };
        let guild_id_i64 = guild_id.get_i64();
        let user_id = ctx.interaction.author_id().unwrap().get_i64();

        let guild = DbGuild::create(&ctx.bot.pool, guild_id_i64).await?;
        let guild = match guild {
            Some(guild) => guild,
            None => DbGuild::get(&ctx.bot.pool, guild_id_i64).await?.unwrap(),
        };

        let mut conf = if let Some(end) = guild.premium_end {
            lang.premium_end_premium_until(end.timestamp())
        } else {
            lang.premium_end_no_premium().to_string()
        };

        conf.push_str("\n\n");
        conf.push_str(&lang.premium_redeem_confirm(
            self.months * constants::CREDITS_PER_MONTH as i64,
            self.months,
        ));

        let ret = confirm::simple(&mut ctx, conf, false).await?;
        let Some(mut btn_ctx) = ret else {
            return Ok(());
        };

        let ret = redeem_premium(
            &ctx.bot,
            user_id,
            guild_id_i64,
            self.months as u64,
            Some(guild.premium_end),
        )
        .await?;

        let resp = match ret {
            RedeemPremiumResult::Ok => lang.premium_redeem_done(),
            RedeemPremiumResult::StateMismatch => lang.premium_redeem_state_mismatch(),
            RedeemPremiumResult::TooFewCredits => lang.premium_redeem_too_few_credits(),
        };
        btn_ctx.edit_str(resp, true).await?;

        Ok(())
    }
}
