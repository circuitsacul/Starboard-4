use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{errors::StarboardResult, interactions::context::CommandCtx};

#[derive(CommandModel, CreateCommand)]
#[command(name = "create-filter", desc = "Create a filter for a filter group.")]
pub struct CreateFilter {
    /// The filter group to create this filter for.
    #[command(autocomplete = true)]
    group: String,
    /// The position to put the filter in. Use 1 for the start (top) or leave blank for the end.
    #[command(min_value = 1)]
    position: Option<i64>,
}

impl CreateFilter {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        ctx.respond_str(&self.group, true).await?;
        Ok(())
    }
}
