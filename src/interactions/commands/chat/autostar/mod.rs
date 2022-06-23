pub mod create;

use anyhow::Result;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::interactions::commands::context::CommandCtx;

use create::CreateAutoStarChannel;

#[derive(CommandModel, CreateCommand)]
#[command(name = "autostar", desc = "Manage autostar channels.")]
pub enum AutoStar {
    #[command(name = "create")]
    Create(CreateAutoStarChannel),
}

impl AutoStar {
    pub async fn callback(self, ctx: CommandCtx) -> Result<()> {
        match self {
            Self::Create(cmd) => cmd.callback(ctx).await?,
        }

        Ok(())
    }
}
