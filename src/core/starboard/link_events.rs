use twilight_model::{
    gateway::payload::incoming::MessageUpdate,
    id::{marker::MessageMarker, Id},
};

use crate::{
    client::bot::StarboardBot, database::Message as DbMessage, errors::StarboardResult,
    utils::id_as_i64::GetI64,
};

use super::handle::RefreshMessage;

pub async fn handle_message_update(
    bot: &StarboardBot,
    event: Box<MessageUpdate>,
) -> StarboardResult<()> {
    let msg = match DbMessage::get_original(&bot.pool, event.id.get_i64()).await? {
        Some(msg) => msg,
        None => return Ok(()),
    };

    let mut refresh = RefreshMessage::new(bot, event.id);
    refresh.set_sql_message(msg);
    refresh.refresh(true).await?;

    Ok(())
}

pub async fn handle_message_delete(
    bot: &StarboardBot,
    message_id: Id<MessageMarker>,
) -> StarboardResult<()> {
    let msg = match DbMessage::get_original(&bot.pool, message_id.get_i64()).await? {
        Some(msg) => msg,
        None => return Ok(()),
    };

    let mut refresh = RefreshMessage::new(bot, message_id);
    refresh.set_sql_message(msg);
    refresh.refresh(true).await?;

    Ok(())
}
