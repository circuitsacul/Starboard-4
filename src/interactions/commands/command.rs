use anyhow::Result;
use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::interactions::commands::context::CommandCtx;

#[async_trait]
pub trait AppCommand: CreateCommand + CommandModel {
    async fn callback(&self, ctx: CommandCtx) -> Result<()>;
}
