mod disable;
mod enable;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{errors::StarboardResult, interactions::context::CommandCtx, locale_func};

locale_func!(autoredeem);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "autoredeem",
    desc = "Manage autoredeem.",
    desc_localizations = "autoredeem"
)]
pub enum Autoredeem {
    #[command(name = "disable")]
    Disable(disable::Disable),
    #[command(name = "enable")]
    Enable(enable::Enable),
}

impl Autoredeem {
    pub async fn callback(self, ctx: CommandCtx) -> StarboardResult<()> {
        match self {
            Self::Disable(cmd) => cmd.callback(ctx).await,
            Self::Enable(cmd) => cmd.callback(ctx).await,
        }
    }
}
