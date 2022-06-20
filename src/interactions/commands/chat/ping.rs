use crate::interactions::commands::{command::AppCommand, context::CommandCtx};

use async_trait::async_trait;
use twilight_util::builder::command::CommandBuilder;
use twilight_model::application::command::{Command, CommandType};

pub struct PingCommand;


#[async_trait]
impl AppCommand for PingCommand {
    fn describe(&self) -> Command {
        CommandBuilder::new(
            "ping".to_string(),
            "Pong!".to_string(),
            CommandType::ChatInput,
        ).build()
    }

    async fn callback(&self, ctx: CommandCtx) {
        let client = ctx.bot.interaction_client().await.unwrap();
        println!("Got client: {:?}", client);
    }
}
