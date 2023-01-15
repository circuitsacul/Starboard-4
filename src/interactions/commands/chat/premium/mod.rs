mod autoredeem;
mod info;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{errors::StarboardResult, interactions::context::CommandCtx};

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "premium",
    desc = "Premium-releated commands. See /premium-locks for locks."
)]
pub enum Premium {
    #[command(name = "info")]
    Info(info::Info),
    #[command(name = "autoredeem")]
    AutoRedeem(autoredeem::AutoRedeem),
}

impl Premium {
    pub async fn callback(self, ctx: CommandCtx) -> StarboardResult<()> {
        match self {
            Self::Info(cmd) => cmd.callback(ctx).await,
            Self::AutoRedeem(cmd) => cmd.callback(ctx).await,
        }
    }
}
