mod refresh;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{errors::StarboardResult, interactions::context::CommandCtx};

#[derive(CommandModel, CreateCommand)]
#[command(name = "locks", desc = "Manage premium locks.")]
pub enum Locks {
    #[command(name = "refresh")]
    Refresh(refresh::Refresh),
}

impl Locks {
    pub async fn callback(self, ctx: CommandCtx) -> StarboardResult<()> {
        match self {
            Self::Refresh(cmd) => cmd.callback(ctx).await,
        }
    }
}
