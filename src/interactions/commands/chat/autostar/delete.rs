use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::AutoStarChannel,
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    utils::{id_as_i64::GetI64, views::confirm},
};

#[derive(CreateCommand, CommandModel)]
#[command(name = "delete", desc = "Delete an autostar channel.")]
pub struct DeleteAutoStarChannel {
    /// The name of the autostar channel to delete.
    #[command(autocomplete = true)]
    name: String,
}

impl DeleteAutoStarChannel {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);

        let mut btn_ctx = match confirm::simple(
            &mut ctx,
            &format!(
                "Are you sure you want to delete the autostar channel '{}'?",
                self.name
            ),
            true,
        )
        .await?
        {
            None => return Ok(()),
            Some(btn_ctx) => btn_ctx,
        };

        let ret = AutoStarChannel::delete(&ctx.bot.pool, &self.name, guild_id.get_i64()).await?;
        if ret.is_none() {
            btn_ctx
                .edit_str("No autostar channel with that name was found.", true)
                .await?;
        } else {
            ctx.bot
                .cache
                .guild_autostar_channel_names
                .remove(&guild_id)
                .await;
            btn_ctx
                .edit_str(&format!("Deleted autostar channel '{}'.", self.name), true)
                .await?;
        }
        Ok(())
    }
}
