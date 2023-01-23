use std::sync::Arc;

use twilight_model::{
    gateway::payload::incoming::MessageUpdate,
    id::{marker::MessageMarker, Id},
};

use crate::{
    client::bot::StarboardBot,
    database::{DbMessage, StarboardMessage},
    errors::StarboardResult,
    utils::id_as_i64::GetI64,
};

use super::handle::RefreshMessage;

pub async fn handle_message_update(
    bot: Arc<StarboardBot>,
    event: Box<MessageUpdate>,
) -> StarboardResult<()> {
    let msg = match DbMessage::get(&bot.pool, event.id.get_i64()).await? {
        Some(msg) => msg,
        None => return Ok(()),
    };

    let mut refresh = RefreshMessage::new(bot, event.id);
    refresh.set_sql_message(msg);
    refresh.refresh(true).await?;

    Ok(())
}

pub async fn handle_message_delete(
    bot: Arc<StarboardBot>,
    message_id: Id<MessageMarker>,
) -> StarboardResult<()> {
    let message_id_i64 = message_id.get_i64();
    let msg = match DbMessage::get_original(&bot.pool, message_id_i64).await? {
        Some(msg) => msg,
        None => return Ok(()),
    };

    if message_id != msg.message_id {
        // this means that a starboard message was deleted, so we want to remove that
        // from the database so that the affected starboard can resend it without
        // needing force=true
        StarboardMessage::delete(&bot.pool, message_id_i64).await?;
    }

    let mut refresh = RefreshMessage::new(bot, message_id);
    refresh.set_sql_message(msg);
    refresh.refresh(false).await?;

    Ok(())
}
