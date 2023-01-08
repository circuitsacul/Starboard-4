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
    name = "refresh",
    desc = "Refresh a message (does not recount reactions)."
)]
pub struct Refresh {
    /// Link to the message to refresh.
    message: String,
}

impl Refresh {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();

        let Some((_channel_id, message_id)) = parse_message_link(&self.message) else {
            ctx.respond_str("Invalid message link.", true).await?;
            return Ok(());
        };

        let Some(orig) = Message::get_original(&ctx.bot.pool, message_id).await? else {
            ctx.respond_str(INVALID_MESSAGE_ERR, true).await?;
            return Ok(());
        };

        if orig.guild_id != guild_id {
            ctx.respond_str("That message belongs to a different server.", true)
                .await?;
            return Ok(());
        }

        ctx.defer(true).await?;

        let ret = RefreshMessage::new(&ctx.bot, message_id.into_id())
            .refresh(true)
            .await?;

        let resp = if ret.map_or(false, |ret| !ret.is_empty()) {
            concat!(
                "Some starboards failed to update. I might be missing permission to send ",
                "messages in the starboard channel, or I might be missing permission to ",
                "view the channel that the original message is in.",
            )
        } else {
            "Message refreshed."
        };

        ctx.respond_str(resp, true).await?;

        Ok(())
    }
}
