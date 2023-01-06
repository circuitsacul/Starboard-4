use std::fmt::Write;

use thousands::Separable;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::XPRole,
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    utils::{embed, id_as_i64::GetI64},
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "view", desc = "View all of your XP-based award roles.")]
pub struct View;

impl View {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();

        let xproles = XPRole::list_by_guild(&ctx.bot.pool, guild_id).await?;
        if xproles.is_empty() {
            ctx.respond_str("There are no XPRoles.", true).await?;
            return Ok(());
        }

        let mut desc = String::new();
        for xpr in xproles {
            writeln!(
                desc,
                "<@&{}> - `{}` XP",
                xpr.role_id,
                xpr.required.separate_with_commas()
            )
            .unwrap();
        }

        let emb = embed::build()
            .title("XP-based Award Roles")
            .description(desc)
            .build();

        ctx.respond(ctx.build_resp().embeds([emb]).build()).await?;

        Ok(())
    }
}
