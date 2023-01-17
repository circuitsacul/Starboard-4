use std::borrow::Cow;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    core::premium::redeem::{redeem_premium, RedeemPremiumResult},
    database::Guild,
    errors::StarboardResult,
    interactions::context::CommandCtx,
    map_dup_none,
    utils::{id_as_i64::GetI64, views::confirm},
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "redeem", desc = "Redeem your premium credits.")]
pub struct Redeem {
    /// The number of months of premium to redeem. Each month is three credits.
    #[command(min_value = 1, max_value = 6)]
    months: i64,
}

impl Redeem {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let Some(guild_id) = ctx.interaction.guild_id else {
            ctx.respond_str("Please run this command in the server you want premium for.", true).await?;
            return Ok(());
        };
        let guild_id_i64 = guild_id.get_i64();
        let user_id = ctx.interaction.author_id().unwrap().get_i64();

        let guild = map_dup_none!(Guild::create(&ctx.bot.pool, guild_id_i64))?;
        let guild = match guild {
            Some(guild) => guild,
            None => Guild::get(&ctx.bot.pool, guild_id_i64).await?.unwrap(),
        };

        let end_pretty = if let Some(end) = guild.premium_end {
            Cow::Owned(format!(
                "This server has premium until <t:{}:F>.",
                end.timestamp()
            ))
        } else {
            Cow::Borrowed("This server does not have premium.")
        };

        let ret = confirm::simple(
            &mut ctx,
            &format!(
                concat!(
                    "{}. Doing this will will add {1} months (each \"month\" is 31 days), and ",
                    "cost you {1} credits. Do you wish to continue?"
                ),
                end_pretty, self.months
            ),
            false,
        )
        .await?;
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
            RedeemPremiumResult::Ok => "Done.",
            RedeemPremiumResult::StateMismatch => concat!(
                "This server's premium status changed while you were running the command. ",
                "Please try again."
            ),
            RedeemPremiumResult::TooFewCredits => "You don't have enough credits.",
        };
        btn_ctx.edit_str(resp, true).await?;

        Ok(())
    }
}
