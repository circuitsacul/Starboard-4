pub mod create;
pub mod delete;

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
    #[command(name = "delete")]
    Delete(delete::DeleteAutoStarChannel),
}

impl AutoStar {
    pub async fn callback(self, ctx: CommandCtx) -> anyhow::Result<()> {
        match self {
            Self::Create(cmd) => cmd.callback(ctx).await,
            Self::Delete(cmd) => cmd.callback(ctx).await,
        }
    }
}
