mod create_filter;
mod create_group;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    errors::StarboardResult,
    interactions::{commands::permissions::manage_roles_channels, context::CommandCtx},
};

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "filters",
    desc = "Manage filters.",
    default_permissions = "manage_roles_channels",
    dm_permission = false
)]
pub enum Filters {
    #[command(name = "create-group")]
    CreateGroup(create_group::CreateGroup),
    #[command(name = "create-filter")]
    CreateFilter(create_filter::CreateFilter),
}

impl Filters {
    pub async fn callback(self, ctx: CommandCtx) -> StarboardResult<()> {
        match self {
            Self::CreateGroup(cmd) => cmd.callback(ctx).await,
            Self::CreateFilter(cmd) => cmd.callback(ctx).await,
        }
    }
}
