use std::sync::Arc;

use async_trait::async_trait;
use twilight_model::application::{interaction::ApplicationCommand, command::Command};

use crate::client::bot::Starboard;

#[derive(Debug)]
pub struct CommandCtx {
    pub shard_id: u64,
    pub bot: Arc<Starboard>,
    pub command: Box<ApplicationCommand>,
}

#[async_trait]
pub trait AppCommand {
    fn info(&self) -> Command;
    async fn execute(&self, mut ctx: CommandCtx) -> Result<(), String>;
}

pub async fn handle_command(shard_id: u64, bot: Arc<Starboard>, command: Box<ApplicationCommand>) {
    let ctx = CommandCtx {
        shard_id,
        bot,
        command,
    };

    println!("Command! {:?}", ctx);
}
