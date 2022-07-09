pub mod create;
pub mod delete;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::interactions::commands::{context::CommandCtx, permissions::manage_channels};

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "starboards",
    desc = "Manage starboards.",
    dm_permission = false,
    default_permissions = "manage_channels"
)]
pub enum Starboard {
    #[command(name = "create")]
    Create(create::CreateStarboard),
    #[command(name = "delete")]
    Delete(delete::DeleteStarboard),
}

impl Starboard {
    pub async fn callback(self, ctx: CommandCtx) -> anyhow::Result<()> {
        match self {
            Self::Create(cmd) => cmd.callback(ctx).await,
            Self::Delete(cmd) => cmd.callback(ctx).await,
        }
    }
}
