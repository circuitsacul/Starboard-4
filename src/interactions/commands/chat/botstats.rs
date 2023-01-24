use thousands::Separable;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_util::builder::embed::EmbedFieldBuilder;

use crate::{errors::StarboardResult, interactions::context::CommandCtx, utils::embed};

#[derive(CommandModel, CreateCommand)]
#[command(name = "botstats", desc = "Show bot statistics.")]
pub struct BotStats;

impl BotStats {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guilds = ctx.bot.cache.guilds.len();
        let cached_users = ctx.bot.cache.users.len();
        let cached_messages = ctx.bot.cache.messages.len();

        let emb = embed::build()
            .title("Starboard's Stats")
            .description(format!("Total Servers: {}", guilds.separate_with_commas(),))
            .field(EmbedFieldBuilder::new(
                "Cache",
                format!(
                    "Cached Users: {}\nCached Messages: {}",
                    cached_users.separate_with_spaces(),
                    cached_messages.separate_with_commas(),
                ),
            ))
            .build();
        ctx.respond(ctx.build_resp().embeds([emb]).build()).await?;

        Ok(())
    }
}
