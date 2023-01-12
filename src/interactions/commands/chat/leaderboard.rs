use std::fmt::Write;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::Member,
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    utils::{embed, id_as_i64::GetI64, views::paginator},
};

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "leaderboard",
    desc = "Show the servers leaderboard.",
    dm_permission = false
)]
pub struct Leaderboard {
    /// Whether to include users who've left. False by default.
    #[command(rename = "include-gone")]
    include_gone: Option<bool>,
}

impl Leaderboard {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();

        let include_gone = self.include_gone == Some(true);

        let lb = if include_gone {
            Member::list_by_xp(&ctx.bot.pool, guild_id, 99).await?
        } else {
            Member::list_by_xp_exclude_deleted(&ctx.bot.pool, guild_id, 99, &ctx.bot.cache).await?
        };
        let mut pages = Vec::new();
        let mut current_page = String::new();

        for (idx, member) in lb.into_iter().enumerate() {
            if idx % 9 == 0 && idx != 0 {
                pages.push(current_page);
                current_page = String::new();
            }

            writeln!(
                current_page,
                "`#{}` <@{}> - {} XP",
                idx + 1,
                member.user_id,
                member.xp
            )
            .unwrap();
        }
        pages.push(current_page);

        let author_id = ctx.interaction.author_id().unwrap();
        paginator::simple(
            &mut ctx,
            pages
                .into_iter()
                .map(|p| {
                    (
                        None,
                        Some(vec![embed::build()
                            .title(if include_gone {
                                "Leaderboard (Including Gone)"
                            } else {
                                "Leaderboard"
                            })
                            .description(p)
                            .build()]),
                    )
                })
                .collect(),
            author_id,
            false,
        )
        .await?;

        Ok(())
    }
}
