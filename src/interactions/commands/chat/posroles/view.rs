use thousands::Separable;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    core::premium::is_premium::is_guild_premium,
    database::PosRole,
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    locale_func,
    utils::{embed, id_as_i64::GetI64, views::paginator},
};

locale_func!(posroles_view);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "view",
    desc = "View all of your position-based award roles.",
    desc_localizations = "posroles_view"
)]
pub struct View;

impl View {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();
        let lang = ctx.user_lang();

        if !is_guild_premium(&ctx.bot, guild_id, true).await? {
            ctx.respond_str(lang.premium_command(), true).await?;
            return Ok(());
        }

        let posroles = PosRole::list_by_guild(&ctx.bot.pool, guild_id).await?;
        if posroles.is_empty() {
            ctx.respond_str(lang.posroles_view_none(), true).await?;
            return Ok(());
        }

        let mut embeds = Vec::new();

        for chunk in posroles.chunks(10) {
            let mut desc = String::new();
            for xpr in chunk {
                desc.push_str(
                    &lang.posrole_description(xpr.max_members.separate_with_commas(), xpr.role_id),
                );
            }

            let emb = embed::build()
                .title(lang.posroles_title())
                .description(desc)
                .build();

            embeds.push(emb);
        }

        let author_id = ctx.interaction.author_id().unwrap();
        paginator::simple(
            &mut ctx,
            embeds
                .into_iter()
                .map(|emb| (None, Some(vec![emb])))
                .collect(),
            author_id,
            false,
        )
        .await?;

        Ok(())
    }
}
