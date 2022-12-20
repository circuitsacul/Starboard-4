pub mod info;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    errors::StarboardResult,
    interactions::{commands::permissions::manage_messages, context::CommandCtx},
};

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "utils",
    desc = "Utility commands.",
    dm_permission = false,
    default_permissions = "manage_messages"
)]
pub enum Utils {
    #[command(name = "info")]
    Info(info::Info),
}

impl Utils {
    pub async fn callback(self, ctx: CommandCtx) -> StarboardResult<()> {
        match self {
            Self::Info(cmd) => cmd.callback(ctx).await,
        }
    }
}
