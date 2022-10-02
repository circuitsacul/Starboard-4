pub mod behavior;
pub mod embed;
pub mod requirements;
pub mod style;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::interactions::context::CommandCtx;

#[derive(CommandModel, CreateCommand)]
#[command(name = "edit", desc = "Edit an override.")]
pub enum EditOverride {
    #[command(name = "embed")]
    Embed(embed::EditEmbedStyle),
    #[command(name = "style")]
    Style(style::EditGeneralStyle),
    #[command(name = "requirements")]
    Requirements(requirements::EditRequirements),
    #[command(name = "behavior")]
    Behaviour(behavior::EditBehavior),
}

impl EditOverride {
    pub async fn callback(self, ctx: CommandCtx) -> anyhow::Result<()> {
        match self {
            Self::Embed(cmd) => cmd.callback(ctx).await,
            Self::Style(cmd) => cmd.callback(ctx).await,
            Self::Requirements(cmd) => cmd.callback(ctx).await,
            Self::Behaviour(cmd) => cmd.callback(ctx).await,
        }
    }
}
