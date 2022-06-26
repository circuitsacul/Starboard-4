use std::sync::Arc;

use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::{
    client::bot::StarboardBot,
    models::{simple_emoji::EmojiCommon, AutoStarChannel, SimpleEmoji},
    unwrap_id,
};

pub async fn handle(bot: Arc<StarboardBot>, event: Box<MessageCreate>) -> anyhow::Result<()> {
    let guild_id: i64 = match event.guild_id {
        None => return Ok(()),
        Some(guild_id) => unwrap_id!(guild_id),
    };
    let channel_id = unwrap_id!(event.channel_id);

    // TODO cache autostar channels per-guild
    let asc = AutoStarChannel::list_by_channel(&bot.pool, channel_id).await?;

    // TODO handle settings
    for a in asc.into_iter() {
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
