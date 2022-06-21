use async_trait::async_trait;
use anyhow::Result;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::interactions::commands::command::AppCommand;
use crate::interactions::commands::context::CommandCtx;

#[derive(CreateCommand, CommandModel)]
#[command(name = "ping", desc = "Pong!")]
pub struct Ping {}

#[async_trait]
impl AppCommand for Ping {
    async fn callback(&self, ctx: CommandCtx) -> Result<()> {
        let resp = InteractionResponseDataBuilder::new()
            .content("Pong!".into())
            .build();

        ctx.respond(resp).await?;

        Ok(())
    }
}
