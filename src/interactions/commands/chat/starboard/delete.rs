use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::Starboard, get_guild_id, interactions::commands::context::CommandCtx, unwrap_id,
};

#[derive(CreateCommand, CommandModel)]
#[command(name = "delete", desc = "Delete a starboard.")]
pub struct DeleteStarboard {
    /// The name of the starboard to delete.
    #[command(autocomplete = true)]
    name: String,
}

impl DeleteStarboard {
    pub async fn callback(self, mut ctx: CommandCtx) -> anyhow::Result<()> {
        let guild_id = get_guild_id!(ctx);
        let ret = Starboard::delete(&ctx.bot.pool, &self.name, unwrap_id!(guild_id)).await?;
        if ret.is_none() {
            ctx.respond_str("No starboard with that name was found.", true)
                .await?;
        } else {
            ctx.bot
                .cache
                .guild_autostar_channel_names
                .remove(&guild_id)
                .await;
            ctx.respond_str(&format!("Deleted starboard '{}'.", self.name), false)
                .await?;
        }
        Ok(())
    }
}
