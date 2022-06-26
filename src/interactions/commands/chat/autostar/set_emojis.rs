use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    core::emoji::{EmojiCommon, SimpleEmoji},
    get_guild_id,
    interactions::commands::context::CommandCtx,
    models::AutoStarChannel,
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "set-emojis", desc = "Set the emojis for an autostar channel.")]
pub struct SetAutostarEmojis {
    /// The name of the autostar channel to edit.
    name: String,
    /// The emojis to use. Use "none" to set to none.
    emojis: String,
}

impl SetAutostarEmojis {
    pub async fn callback(self, ctx: CommandCtx) -> anyhow::Result<()> {
        let guild_id = get_guild_id!(ctx);

        let emojis = Vec::<SimpleEmoji>::from_user_input(self.emojis, &ctx.bot, guild_id).await;
        let asc = AutoStarChannel::set_emojis(&ctx.bot.pool, &self.name, guild_id, emojis).await?;

        if asc.is_none() {
            ctx.respond_str("No autostar channels with that name were found.", true)
                .await?;
            return Ok(());
        }

        // set the emojis
        ctx.respond_str(&format!("Set the emojis for '{}'.", self.name,), false)
            .await?;

        Ok(())
    }
}
