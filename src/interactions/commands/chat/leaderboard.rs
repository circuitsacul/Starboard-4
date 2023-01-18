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

        let mut idx = 0;
        let pages = lb.chunks(9).map(|chunk| {
            chunk
                .iter()
                .map(|Member { user_id, xp, .. }| {
                    idx += 1;
                    format!("`#{idx}` <@{user_id}> - {xp} XP\n")
                })
                .collect::<String>()
        });

        let author_id = ctx.interaction.author_id().unwrap();
        paginator::simple(
            &mut ctx,
            pages
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
