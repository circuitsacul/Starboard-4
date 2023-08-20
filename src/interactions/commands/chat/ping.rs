use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{errors::StarboardResult, interactions::context::CommandCtx};

#[derive(CreateCommand, CommandModel)]
#[command(name = "ping", desc = "Pong!")]
pub struct Ping;

impl Ping {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        ctx.respond_str(ctx.user_lang().pong(), false).await?;

        Ok(())
    }
}
