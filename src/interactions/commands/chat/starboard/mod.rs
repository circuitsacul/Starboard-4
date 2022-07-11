pub mod create;
pub mod delete;
pub mod view;

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
    #[command(name = "view")]
    View(view::ViewStarboard),
}

impl Starboard {
    pub async fn callback(self, ctx: CommandCtx) -> anyhow::Result<()> {
        match self {
            Self::Create(cmd) => cmd.callback(ctx).await,
            Self::Delete(cmd) => cmd.callback(ctx).await,
            Self::View(cmd) => cmd.callback(ctx).await,
        }
    }
}
