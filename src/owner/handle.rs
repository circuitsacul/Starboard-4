use twilight_model::id::{
    marker::{ChannelMarker, MessageMarker},
    Id,
};

use crate::{
    cache::models::message::CachedMessage, client::bot::StarboardBot, errors::StarboardResult,
};

use super::commands;

pub async fn handle_message(
    _shard_id: u64,
    bot: &StarboardBot,
    channel_id: Id<ChannelMarker>,
    message_id: Id<MessageMarker>,
    message: &CachedMessage,
    is_edit: bool,
) -> StarboardResult<()> {
    // first check that this is a command being run by the bot owner
    if !bot.config.owner_ids.contains(&message.author_id.get()) {
        return Ok(());
    }

    // split by space
    let tokens: Vec<_> = message.content.trim().split([' ', '\n']).collect();

    // need at least two tokens
    if tokens.len() < 2 {
        return Ok(());
    }

    // first token should be a prefix
    if tokens[0].to_lowercase().trim() != "star" {
        return Ok(());
    }

    // match second token to a command, if any
    let ret = match tokens[1] {
        "sql" => commands::sql::run_sql(bot, channel_id, message_id, message, is_edit).await,
        // "embed" => commands::embed_test::test_starboard_embed(bot, event).await?,
        _ => Ok(()),
    };

    if let Err(err) = ret {
        bot.http
            .create_message(channel_id)
            .content(&err.to_string())?
            .await?;
    }

    Ok(())
}
