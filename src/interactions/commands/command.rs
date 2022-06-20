use async_trait::async_trait;

use twilight_model::application::command::Command;

use crate::interactions::commands::context::CommandCtx;

#[async_trait]
pub trait AppCommand: Send + Sync + 'static {
    fn describe(&self) -> Command;
    async fn callback(&self, ctx: CommandCtx);
}
