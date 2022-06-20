use async_trait::async_trait;

use twilight_model::application::command::Command;

use crate::interactions::commands::context::CommandCtx;

#[async_trait]
pub trait AppCommand {
    fn info(&self) -> Command;
    async fn execute(&self, mut ctx: CommandCtx) -> Result<(), String>;
}
