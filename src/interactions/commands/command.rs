use anyhow::Result;
use async_trait::async_trait;

use twilight_interactions::command::{CommandInputData, CommandModel, CreateCommand};

use crate::interactions::commands::context::CommandCtx;

#[async_trait]
pub trait AppCommand: CreateCommand + CommandModel + Send + Sync {
    async fn callback(self, ctx: CommandCtx) -> Result<()>;
    async fn handle(data: CommandInputData<'_>, ctx: CommandCtx) -> Result<()> {
        Self::from_interaction(data)?.callback(ctx).await
    }
}

#[async_trait]
pub trait GroupCommand: CreateCommand + CommandModel + Send + Sync {
    async fn call_callback(&self, ctx: CommandCtx) -> Result<()>;
    async fn handle(data: CommandInputData<'_>, ctx: CommandCtx) -> Result<()> {
        Self::from_interaction(data)?.call_callback(ctx).await
    }
}
