use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::client::bot::StarboardBot;

pub async fn handle_message(shard_id: u64, bot: &StarboardBot, event: &MessageCreate) {
    // first check that this is a command being run by the bot owner
    if !bot.config.owner_ids.contains(&event.author.id.get()) {
        return;
    }

    bot.http
        .create_message(event.channel_id)
        .content(&format!("{}", shard_id))
        .unwrap()
        .exec()
        .await
        .unwrap();
}
