use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::Starboard, get_guild_id, interactions::context::CommandCtx, unwrap_id,
    utils::views::confirm,
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

        let mut int_ctx = match confirm::simple(&mut ctx, "Are you sure?", true).await? {
            None => return Ok(()),
            Some(int_ctx) => int_ctx,
        };

        let ret = Starboard::delete(&ctx.bot.pool, &self.name, unwrap_id!(guild_id)).await?;
        if ret.is_none() {
            int_ctx
                .edit_str("No starboard with that name was found.", true)
                .await?;
        } else {
            ctx.bot
                .cache
                .guild_autostar_channel_names
                .remove(&guild_id)
                .await;
            ctx.bot
                .cache
                .guild_vote_emojis
                .remove(&unwrap_id!(guild_id));
            int_ctx
                .edit_str(&format!("Deleted starboard '{}'.", self.name), true)
                .await?;
        }
        Ok(())
    }
}
