use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::{
    client::bot::StarboardBot,
    core::emoji::{EmojiCommon, SimpleEmoji},
    models::AutoStarChannel,
    unwrap_id,
};

pub async fn handle(bot: StarboardBot, event: Box<MessageCreate>) -> anyhow::Result<()> {
    // Ignore DMs
    if event.guild_id.is_none() {
        return Ok(());
    }

    // Check the cache...
    if !bot.autostar_channel_ids.contains(&event.channel_id) {
        return Ok(());
    }

    // Fetch the autostar channels
    let asc = AutoStarChannel::list_by_channel(&bot.pool, unwrap_id!(event.channel_id)).await?;

    // If none, remove the channel id from the cache
    if asc.len() == 0 {
        bot.autostar_channel_ids.remove(&event.channel_id);
        return Ok(());
    }

    // Handle the autostar channels
    for a in asc.into_iter() {
        // TODO handle settings
        for emoji in Vec::<SimpleEmoji>::from_stored(a.emojis) {
            let _ = bot
                .http
                .create_reaction(event.channel_id, event.id, &emoji.reactable())
                .exec()
                .await;
        }
    }

    Ok(())
}
