mod add;
mod remove;

use crate::{errors::StarboardResult, interactions::context::CommandCtx, locale_func};
use twilight_interactions::command::{CommandModel, CreateCommand};

locale_func!(autostar_filters);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "filters",
    desc = "Manage filter groups for an autostar channel.",
    desc_localizations = "autostar_filters"
)]
pub enum Filters {
    #[command(name = "add")]
    Add(add::Add),
    #[command(name = "remove")]
    Remove(remove::Remove),
}

impl Filters {
    pub async fn callback(self, ctx: CommandCtx) -> StarboardResult<()> {
        match self {
            Self::Add(cmd) => cmd.callback(ctx).await,
            Self::Remove(cmd) => cmd.callback(ctx).await,
        }
    }
}
