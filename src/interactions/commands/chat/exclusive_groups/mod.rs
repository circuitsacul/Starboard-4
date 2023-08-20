mod create;
mod delete;
mod rename;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    errors::StarboardResult,
    interactions::{commands::permissions::manage_channels, context::CommandCtx},
    locale_func,
};

locale_func!(exclusive_groups);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "exclusive-groups",
    desc = "Manage exclusive groups for starboards.",
    desc_localizations = "exclusive_groups",
    default_permissions = "manage_channels",
    dm_permission = false
)]
pub enum ExclusiveGroups {
    #[command(name = "create")]
    Create(create::Create),
    #[command(name = "delete")]
    Delete(delete::Delete),
    #[command(name = "rename")]
    Rename(rename::Rename),
}

impl ExclusiveGroups {
    pub async fn callback(self, ctx: CommandCtx) -> StarboardResult<()> {
        match self {
            Self::Create(cmd) => cmd.callback(ctx).await,
            Self::Delete(cmd) => cmd.callback(ctx).await,
            Self::Rename(cmd) => cmd.callback(ctx).await,
        }
    }
}
