use anyhow::Result;
use async_trait::async_trait;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::interactions::commands::command::AppCommand;
use crate::interactions::commands::context::CommandCtx;

#[derive(CreateCommand, CommandModel)]
#[command(name = "ping", desc = "Pong!")]
pub struct Ping {}

#[async_trait]
impl AppCommand for Ping {
    async fn callback(self, ctx: CommandCtx) -> Result<()> {
        let latency = ctx.bot.cluster.info()[&ctx.shard_id].latency().average();
        let millis = match latency {
            None => "Unkown latency.".to_string(),
            Some(duration) => format!("{}ms latency.", duration.as_millis()),
        };
        let resp = InteractionResponseDataBuilder::new()
            .content(format!("Pong! {}", millis))
            .build();

        ctx.respond(resp).await?;

        Ok(())
    }
}
