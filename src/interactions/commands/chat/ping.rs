use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::interactions::context::CommandCtx;

#[derive(CreateCommand, CommandModel)]
#[command(name = "ping", desc = "Pong!")]
pub struct Ping;

impl Ping {
    pub async fn callback(self, mut ctx: CommandCtx) -> anyhow::Result<()> {
        let latency = ctx.bot.cluster.info()[&ctx.shard_id].latency().average();
        let millis = match latency {
            None => "Unknown latency.".to_string(),
            Some(duration) => format!("{}ms latency.", duration.as_millis()),
        };
        ctx.respond_str(&format!("Pong! {}", millis), false).await?;

        Ok(())
    }
}
