mod remove;
mod set;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::interactions::context::CommandCtx;

#[derive(CommandModel, CreateCommand)]
#[command(name = "channels", desc = "Manage the channels an override affects.")]
pub enum ManageOverrideChannels {
    #[command(name = "set")]
    Set(set::SetOverrideChannels),
    #[command(name = "remove")]
    Remove(remove::RemoveOverrideChannels),
}

impl ManageOverrideChannels {
    pub async fn callback(self, ctx: CommandCtx) -> anyhow::Result<()> {
        match self {
            Self::Set(cmd) => cmd.callback(ctx).await,
            Self::Remove(cmd) => cmd.callback(ctx).await,
        }
    }
}
