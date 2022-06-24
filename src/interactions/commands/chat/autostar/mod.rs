pub mod create;

use anyhow::Result;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::interactions::commands::context::CommandCtx;
use crate::interactions::commands::permissions::manage_channels;

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "autostar",
    desc = "Manage autostar channels.",
    dm_permission = false,
    default_permissions = "manage_channels"
)]
pub enum AutoStar {
    #[command(name = "create")]
    Create(create::CreateAutoStarChannel),
}

impl AutoStar {
    pub async fn callback(self, ctx: CommandCtx) -> Result<()> {
        match self {
            Self::Create(cmd) => cmd.callback(ctx).await?,
        }

        Ok(())
    }
}
