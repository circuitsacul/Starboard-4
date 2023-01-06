use std::fmt::Write;

use thousands::Separable;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::PosRole,
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    utils::{embed, id_as_i64::GetI64},
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "view", desc = "View all of your position-based award roles.")]
pub struct View;

impl View {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();

        let posroles = PosRole::list_by_guild(&ctx.bot.pool, guild_id).await?;
        if posroles.is_empty() {
            ctx.respond_str("There are no PosRoles.", true).await?;
            return Ok(());
        }

        let mut desc = String::new();
        for xpr in posroles {
            writeln!(
                desc,
                "<@&{}> - `{}` members",
                xpr.role_id,
                xpr.max_members.separate_with_commas()
            )
            .unwrap();
        }

        let emb = embed::build()
            .title("Position-based Award Roles")
            .description(desc)
            .build();

        ctx.respond(ctx.build_resp().embeds([emb]).build()).await?;

        Ok(())
    }
}
