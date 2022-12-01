use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::{client::bot::StarboardBot, errors::StarboardResult};

use super::commands;

pub async fn handle_message(
    _shard_id: u64,
    bot: &StarboardBot,
    event: &MessageCreate,
) -> StarboardResult<()> {
    // first check that this is a command being run by the bot owner
    if !bot.config.owner_ids.contains(&event.author.id.get()) {
        return Ok(());
    }

    // split by space
    let tokens: Vec<_> = event.content.trim().split([' ', '\n']).collect();

    // need at least two tokens
    if tokens.len() < 2 {
        return Ok(());
    }

    // first token should be a prefix
    if tokens[0].to_lowercase().trim() != "star" {
        return Ok(());
    }

    // match second token to a command, if any
    match tokens[1] {
        "sql" => commands::sql::run_sql(bot, event).await?,
        // "embed" => commands::embed_test::test_starboard_embed(bot, event).await?,
        _ => {}
    }

    Ok(())
}
