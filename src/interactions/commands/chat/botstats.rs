use thousands::Separable;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{errors::StarboardResult, interactions::context::CommandCtx, utils::embed};

#[derive(CommandModel, CreateCommand)]
#[command(name = "botstats", desc = "Show bot statistics.")]
pub struct BotStats;

impl BotStats {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guilds = ctx.bot.cache.guilds.len();
        let users = ctx.bot.cache.users.len();

        let emb = embed::build()
            .title("Starboard's Stats")
            .description(format!(
                "Servers: {}\nUsers: {}",
                guilds.separate_with_commas(),
                users.separate_with_commas()
            ))
            .build();
        ctx.respond(ctx.build_resp().embeds([emb]).build()).await?;

        Ok(())
    }
}
