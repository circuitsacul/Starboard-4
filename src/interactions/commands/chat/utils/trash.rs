use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    core::starboard::handle::RefreshMessage,
    database::Message,
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    utils::{id_as_i64::GetI64, into_id::IntoId, message_link::parse_message_link},
};

use super::INVALID_MESSAGE_ERR;

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "trash",
    desc = "Trash a message so that it is removed from all starboards."
)]
pub struct Trash {
    /// Link to the message to trash.
    message: String,
}

impl Trash {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);

        let Some((_channel_id, message_id)) = parse_message_link(&self.message) else {
            ctx.respond_str("Invalid message link.", true).await?;
            return Ok(());
        };

        let Some(orig) = Message::get_original(&ctx.bot.pool, message_id).await? else {
            ctx.respond_str(INVALID_MESSAGE_ERR, true).await?;
            return Ok(());
        };

        if orig.guild_id != guild_id.get_i64() {
            ctx.respond_str("That message belongs to a different server.", true)
                .await?;
            return Ok(());
        }

        Message::set_trashed(&ctx.bot.pool, orig.message_id, true).await?;
        ctx.respond_str("Message trashed.", true).await?;
        RefreshMessage::new(&ctx.bot, orig.message_id.into_id())
            .refresh(true)
            .await?;

        Ok(())
    }
}

#[derive(CommandModel, CreateCommand)]
#[command(name = "untrash", desc = "Untrashes a message.")]
pub struct UnTrash {
    /// Link to the message to untrash.
    message: String,
}

impl UnTrash {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);

        let Some((_channel_id, message_id)) = parse_message_link(&self.message) else {
            ctx.respond_str("Invalid message link.", true).await?;
            return Ok(());
        };

        let Some(orig) = Message::get_original(&ctx.bot.pool, message_id).await? else {
            ctx.respond_str(INVALID_MESSAGE_ERR, true).await?;
            return Ok(());
        };

        if orig.guild_id != guild_id.get_i64() {
            ctx.respond_str("That message belongs to a different server.", true)
                .await?;
            return Ok(());
        }

        Message::set_trashed(&ctx.bot.pool, orig.message_id, false).await?;
        ctx.respond_str("Message untrashed.", true).await?;
        RefreshMessage::new(&ctx.bot, orig.message_id.into_id())
            .refresh(true)
            .await?;

        Ok(())
    }
}
