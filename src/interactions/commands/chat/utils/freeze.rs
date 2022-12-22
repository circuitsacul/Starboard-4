use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    core::starboard::handle::RefreshMessage,
    database::Message,
    errors::StarboardResult,
    interactions::context::CommandCtx,
    utils::{into_id::IntoId, message_link::parse_message_link},
};

use super::INVALID_MESSAGE_ERR;

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "freeze",
    desc = "Freeze a message, so that it cannot receive or lose any votes."
)]
pub struct Freeze {
    /// Link to the message to freeze.
    message: String,
}

impl Freeze {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let Some((_channel_id, message_id)) = parse_message_link(&self.message) else {
            ctx.respond_str("Invalid message link.", true).await?;
            return Ok(());
        };

        let Some(orig) = Message::get_original(&ctx.bot.pool, message_id).await? else {
            ctx.respond_str(INVALID_MESSAGE_ERR, true).await?;
            return Ok(());
        };

        Message::set_freeze(&ctx.bot.pool, orig.message_id, true)
            .await?
            .unwrap();
        ctx.respond_str("Message frozen.", true).await?;

        let mut refresh = RefreshMessage::new(&ctx.bot, orig.message_id.into_id());
        refresh.refresh(true).await?;

        Ok(())
    }
}

#[derive(CommandModel, CreateCommand)]
#[command(name = "unfreeze", desc = "Unfreeze a message.")]
pub struct UnFreeze {
    /// Link to the message to unfreeze.
    message: String,
}

impl UnFreeze {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let Some((_channel_id, message_id)) = parse_message_link(&self.message) else {
            ctx.respond_str("Invalid message link.", true).await?;
            return Ok(());
        };

        let Some(orig) = Message::get_original(&ctx.bot.pool, message_id).await? else {
            ctx.respond_str(INVALID_MESSAGE_ERR, true).await?;
            return Ok(());
        };

        Message::set_freeze(&ctx.bot.pool, orig.message_id, false)
            .await?
            .unwrap();
        ctx.respond_str("Message unfrozen.", true).await?;

        let mut refresh = RefreshMessage::new(&ctx.bot, orig.message_id.into_id());
        refresh.refresh(true).await?;

        Ok(())
    }
}
