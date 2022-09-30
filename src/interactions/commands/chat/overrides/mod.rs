mod create;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::interactions::{commands::permissions::manage_channels, context::CommandCtx};

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "overrides",
    desc = "Manage overrides.",
    dm_permission = false,
    default_permissions = "manage_channels"
)]
pub enum Overrides {
    #[command(name = "create")]
    Create(create::CreateOverride),
}

impl Overrides {
    pub async fn callback(self, ctx: CommandCtx) -> anyhow::Result<()> {
        match self {
            Self::Create(cmd) => cmd.callback(ctx).await,
        }
    }
}
