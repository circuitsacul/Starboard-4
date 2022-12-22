pub mod force;
pub mod freeze;
pub mod info;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    errors::StarboardResult,
    interactions::{commands::permissions::manage_messages, context::CommandCtx},
};

const INVALID_MESSAGE_ERR: &str = concat!(
    "I couldn't find that message. There are a few possible reasons why:",
    "\n - I don't have access to the channel the message is in.",
    "\n - The message doesn't exist.",
    "\n - The message doesn't have any upvotes, so it isn't in the database.",
);

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

    #[command(name = "freeze")]
    Freeze(freeze::Freeze),
    #[command(name = "unfreeze")]
    UnFreeze(freeze::UnFreeze),

    #[command(name = "force")]
    Force(force::Force),
}

impl Utils {
    pub async fn callback(self, ctx: CommandCtx) -> StarboardResult<()> {
        match self {
            Self::Info(cmd) => cmd.callback(ctx).await,

            Self::Freeze(cmd) => cmd.callback(ctx).await,
            Self::UnFreeze(cmd) => cmd.callback(ctx).await,

            Self::Force(cmd) => cmd.callback(ctx).await,
        }
    }
}
