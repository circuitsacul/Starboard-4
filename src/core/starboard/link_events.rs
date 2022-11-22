use twilight_model::gateway::payload::incoming::{MessageDelete, MessageUpdate};

use crate::{
    client::bot::StarboardBot, database::Message as DbMessage, errors::StarboardResult, unwrap_id,
};

use super::handle::RefreshMessage;

pub async fn handle_message_update(
    bot: &StarboardBot,
    event: Box<MessageUpdate>,
) -> StarboardResult<()> {
    let msg = match DbMessage::get_original(&bot.pool, unwrap_id!(event.id)).await? {
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
    event: MessageDelete,
) -> StarboardResult<()> {
    let msg = match DbMessage::get_original(&bot.pool, unwrap_id!(event.id)).await? {
        Some(msg) => msg,
        None => return Ok(()),
    };

    let mut refresh = RefreshMessage::new(bot, event.id);
    refresh.set_sql_message(msg);
    refresh.refresh(true).await?;

    Ok(())
}
